// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn piranhacopter_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
    && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)
    && boma.status_frame() >= 30
    {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, false);
    }

    if status_kind == *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END
    && boma.is_motion(Hash40::new("special_hi"))
    {
        if boma.is_prev_situation(*SITUATION_KIND_AIR)
        && boma.is_situation(*SITUATION_KIND_GROUND)
        {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        if boma.is_prev_situation(*SITUATION_KIND_GROUND)
        && boma.is_situation(*SITUATION_KIND_AIR)
        {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(boma,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        let stop_add_speed_y_frame = WorkModule::get_param_int(boma, hash40("param_special_hi"), hash40("stop_add_speed_y_frame"));
        if boma.status_frame() >= stop_add_speed_y_frame {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING, false);
        }
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe fn stance_head(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    // Enable meshes for stances
    // HeadA is the normal head
	// HeadB is the poison head
	// HeadS is the spike head
    if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 0 {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), false);
    }
    else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 1  {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), false);
    }
    else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2  {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), false);
    }
}

unsafe fn stance_init_effects(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.object(), vars::packun::instance::STANCE_INIT) {
        if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 0 {
            LANDING_EFFECT(fighter, Hash40::new("sys_grass"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
        else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 1 {
            LANDING_EFFECT(fighter, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
            LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        VarModule::off_flag(fighter.object(), vars::packun::instance::STANCE_INIT);
    }
}

/// handle speed application
unsafe fn check_apply_speeds(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    
    // handle speed application once
    if VarModule::is_flag(fighter.object(), vars::packun::instance::STANCE_NEED_SET_SPEEDS) {
        if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 0 {
            apply_status_speed_mul(fighter, 1.0);
        } else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 1 {
            apply_status_speed_mul(fighter, 0.88);
        } else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
            apply_status_speed_mul(fighter, 0.75);
        }
        VarModule::off_flag(fighter.object(), vars::packun::instance::STANCE_NEED_SET_SPEEDS);
    }

    if fighter.status() != VarModule::get_int(fighter.object(), vars::packun::instance::STANCE_STATUS) {
        //println!("Status is changing!");
        VarModule::on_flag(fighter.object(), vars::packun::instance::STANCE_NEED_SET_SPEEDS);
        VarModule::set_int(fighter.object(), vars::packun::instance::STANCE_STATUS, fighter.status());
        //println!("new stance status: {}", VarModule::get_int(fighter.object(), vars::packun::instance::STANCE_STATUS));
    }

    // dash & momentum transfer speeds
    if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 1 {
        VarModule::set_float(fighter.object(), vars::common::instance::JUMP_SPEED_MAX_MUL, 0.88);

        // if you are initial dash, slow them down slightly
        if fighter.is_status(*FIGHTER_STATUS_KIND_DASH) {
            let motion_vec = Vector3f {
                x: -0.15 * PostureModule::lr(fighter.boma()) * (1.0 - (MotionModule::frame(fighter.boma()) / MotionModule::end_frame(fighter.boma()))),
                y: 0.0, 
                z: 0.0
            };
            KineticModule::add_speed_outside(fighter.boma(), *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
    }

    else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
        VarModule::set_float(fighter.object(), vars::common::instance::JUMP_SPEED_MAX_MUL, 0.75);

        // if you are initial dash, slow them down slightly
        if fighter.is_status(*FIGHTER_STATUS_KIND_DASH) {
            let motion_vec = Vector3f {
                x: -0.25 * PostureModule::lr(fighter.boma()) * (1.0 - (MotionModule::frame(fighter.boma()) / MotionModule::end_frame(fighter.boma()))),
                y: 0.0, 
                z: 0.0
            };
            KineticModule::add_speed_outside(fighter.boma(), *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
    }
}

/// checks if stance should be reset due to death or match end
unsafe fn check_reset(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]) {
            VarModule::set_int(fighter.object(), vars::packun::instance::CURRENT_STANCE, 0);
    }
}

/// applies the given multiplier to various speed stats of the given fighter. 
/// This should only be called once per status, or you will get some multiplicative effects
unsafe fn apply_status_speed_mul(fighter: &mut smash::lua2cpp::L2CFighterCommon, mul: f32) {
    // set the X motion speed multiplier (where movement is baked into an anim)
    lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter.get_motion_energy(), mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_mul( fighter.get_controller_energy(), mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_add( fighter.get_controller_energy(), mul);

    // set the X speed max multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_speed_max(fighter.get_controller_energy(), mul);
}

unsafe fn sspecial_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE) == *FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_NONE, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE);
                //ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

unsafe fn ptooie_scale(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
        VarModule::set_float(boma.object(), vars::packun::instance::PTOOIE_SCALE, 1.3);
    }
    else {
        VarModule::set_float(boma.object(), vars::packun::instance::PTOOIE_SCALE, 1.0);
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    piranhacopter_cancel(boma, status_kind, situation_kind, cat[0]);
	//spike_head_mesh_test(boma);
    sspecial_cancel(boma, status_kind, situation_kind);
    ptooie_scale(boma);
    stance_head(fighter);
    check_reset(fighter);
    check_apply_speeds(fighter);
    stance_init_effects(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_PACKUN )]
pub fn packun_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		packun_frame(fighter);
    }
}

pub unsafe fn packun_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
