use crate::c_api;
use crate::model::Model;
use crate::variable::Variable;
use crate::retcode::SCIPRetcode;
use std::fmt;
use std::rc::Rc;

pub struct Solution<'a> {
    model: Rc<&'a Model>,
    scip_sol: *mut c_api::SCIP_SOL,
}

impl<'a> Solution<'a> {
    pub fn new(
        scip_ptr: Rc<&'a Model>,
        scip_sol_prt: *mut c_api::SCIP_Sol,
    ) -> Result<Self, SCIPRetcode> {
        Ok(Solution {
            model: scip_ptr,
            scip_sol: scip_sol_prt,
        })
    }

    pub fn get_obj_val(&self) -> f64 {
        unsafe { c_api::SCIPgetSolOrigObj(self.model.scip, self.scip_sol) }
    }

    pub fn get_var_val(&self, var: &Variable) -> f64 {
        unsafe { c_api::SCIPgetSolVal(self.model.scip, self.scip_sol, var.scip_var) }
    }
}

impl<'a> fmt::Debug for Solution<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let obj_val = self.get_obj_val();
        write!(f, "Solution with obj val: {}\n", obj_val)?;
        for var in self.model.get_vars() {
            let val = self.get_var_val(&var);
            if val > 0.0 {
                write!(f, "Var {}={}\n", var.get_name(), val)?;
            }
        }
        Ok(())
    }
}