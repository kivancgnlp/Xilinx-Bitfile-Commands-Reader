use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::TypeDefinitions::{CmdRegs, ConfigRegs};

pub(crate) struct LookupData{
    config_regs_reverse_lookup : HashMap<u8,ConfigRegs>,
    cmd_regs_reverse_lookup: HashMap<u8,CmdRegs>,
    
}


impl LookupData {
    
    fn add_to_config_regs(&mut self, cf: ConfigRegs){
        let result =self.config_regs_reverse_lookup.insert(cf as u8, cf);
        
        if result.is_some(){
            eprintln!("Duplicate entry found at config regs");
        }
        
    }

    fn add_to_cmd_regs(&mut self, cmd_r: CmdRegs){
        let result =self.cmd_regs_reverse_lookup.insert(cmd_r as u8, cmd_r);

        if result.is_some(){
            eprintln!("Duplicate entry found at cmd regs");
        }

    }
    pub(crate) fn new() -> LookupData {
                        
        let mut ld = LookupData{config_regs_reverse_lookup: HashMap::new(), cmd_regs_reverse_lookup: HashMap::new()};
        
   
        ld.add_to_config_regs(ConfigRegs::CRC);
        ld.add_to_config_regs(ConfigRegs::FAR);
        ld.add_to_config_regs(ConfigRegs::FDRI);
        ld.add_to_config_regs(ConfigRegs::FDRO);
        ld.add_to_config_regs(ConfigRegs::CMD);
        ld.add_to_config_regs(ConfigRegs::CTL0);
        ld.add_to_config_regs(ConfigRegs::MASK);
        ld.add_to_config_regs(ConfigRegs::STAT);
        ld.add_to_config_regs(ConfigRegs::LOUT);
        ld.add_to_config_regs(ConfigRegs::COR0);
        ld.add_to_config_regs(ConfigRegs::MFWR);
        ld.add_to_config_regs(ConfigRegs::CBC);
        ld.add_to_config_regs(ConfigRegs::IDCODE);
        ld.add_to_config_regs(ConfigRegs::AXSS);
        ld.add_to_config_regs(ConfigRegs::COR1);
        ld.add_to_config_regs(ConfigRegs::WBSTAR);
        ld.add_to_config_regs(ConfigRegs::TIMER);
        ld.add_to_config_regs(ConfigRegs::RBCRC_SW);
        ld.add_to_config_regs(ConfigRegs::BOOTSTS);
        ld.add_to_config_regs(ConfigRegs::CTL1);
        ld.add_to_config_regs(ConfigRegs::BSPI);
             
        
        ld.add_to_cmd_regs(CmdRegs::NULL);
        ld.add_to_cmd_regs(CmdRegs::WCFG);
        ld.add_to_cmd_regs(CmdRegs::MFW);
        ld.add_to_cmd_regs(CmdRegs::DGHIGH_LFRM);
        ld.add_to_cmd_regs(CmdRegs::RCFG);
        ld.add_to_cmd_regs(CmdRegs::START);
        ld.add_to_cmd_regs(CmdRegs::URAM);
        ld.add_to_cmd_regs(CmdRegs::RCRC);
        ld.add_to_cmd_regs(CmdRegs::AGHIGH);
        ld.add_to_cmd_regs(CmdRegs::SWITCH);
        ld.add_to_cmd_regs(CmdRegs::GRESTORE);
        ld.add_to_cmd_regs(CmdRegs::SHUTDOWN);
        ld.add_to_cmd_regs(CmdRegs::DESYNC);
        ld.add_to_cmd_regs(CmdRegs::IPROG);
        ld.add_to_cmd_regs(CmdRegs::CRCC);
        ld.add_to_cmd_regs(CmdRegs::LTIMER);
        ld.add_to_cmd_regs(CmdRegs::BSPI_READ);
        ld.add_to_cmd_regs(CmdRegs::FALL_EDGE);

        
        ld
    }
    
    pub(crate) fn lookup_config_reg_from_id(&self, id :u8) -> ConfigRegs{
        self.config_regs_reverse_lookup[&id]        
    }

    pub(crate) fn lookup_cmd_reg_from_id(&self, id :u8) -> CmdRegs{
        self.cmd_regs_reverse_lookup[&id]
    }
}


impl Display for LookupData{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let mut keys = Vec::<u8>::new();

        self.config_regs_reverse_lookup.keys().for_each(|k| { keys.push(*k);});
        keys.sort();

        for key in &keys {
            writeln!(f, "Config reg ID {:#x} = {:?}:", key, self.config_regs_reverse_lookup[key])?;
        }

        keys.clear();
        self.cmd_regs_reverse_lookup.keys().for_each(|k| { keys.push(*k);});
        keys.sort();

        for key in keys{
            writeln!(f, "Command reg ID {:#x} = {:?}:", key, self.cmd_regs_reverse_lookup[&key])?;
        }


        Ok(())
    }
}
