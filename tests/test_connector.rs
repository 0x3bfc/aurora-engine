use near_sdk::borsh::BorshSerialize;
use near_sdk::serde_json::json;
use near_sdk::test_utils::accounts;
use near_sdk_sim::{to_yocto, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT};

use aurora_engine::parameters::NewCallArgs;
use aurora_engine::types::EthAddress;

const CONTRACT_ACC: &'static str = "eth_connector.root";
const PROOF_DATA_NEAR: &'static str = r#"{"log_index":3,"log_entry_data":[248,251,148,185,247,33,158,67,78,170,112,33,174,95,158,205,12,171,194,64,84,71,163,248,66,160,91,253,175,236,57,174,146,96,226,220,66,250,35,21,1,244,101,251,175,87,166,187,188,197,23,157,14,86,105,51,218,174,160,0,0,0,0,0,0,0,0,0,0,0,0,137,27,39,73,35,139,39,255,88,233,81,8,142,85,176,77,231,29,195,116,184,160,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,96,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,197,18,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,194,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,114,111,111,116,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"receipt_index":2,"receipt_data":[249,2,7,1,131,4,23,235,185,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,16,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,248,253,248,251,148,185,247,33,158,67,78,170,112,33,174,95,158,205,12,171,194,64,84,71,163,248,66,160,91,253,175,236,57,174,146,96,226,220,66,250,35,21,1,244,101,251,175,87,166,187,188,197,23,157,14,86,105,51,218,174,160,0,0,0,0,0,0,0,0,0,0,0,0,137,27,39,73,35,139,39,255,88,233,81,8,142,85,176,77,231,29,195,116,184,160,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,96,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,197,18,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,194,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,114,111,111,116,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"header_data":[249,2,23,160,38,218,34,66,85,105,115,189,143,118,209,253,91,112,243,84,86,221,182,255,58,218,175,109,4,178,232,20,117,166,136,9,160,29,204,77,232,222,199,93,122,171,133,181,103,182,204,212,26,211,18,69,27,148,138,116,19,240,161,66,253,64,212,147,71,148,133,144,61,184,18,226,104,232,95,87,168,157,222,54,247,146,130,252,104,73,160,250,170,98,144,140,231,40,189,51,132,183,104,161,48,73,186,16,107,80,209,61,81,31,74,150,59,83,7,228,108,245,178,160,64,153,231,0,109,34,81,241,124,239,126,194,51,46,147,136,94,70,172,155,236,69,200,235,252,152,77,9,210,65,9,90,160,204,36,218,251,132,243,193,164,153,49,91,123,27,58,22,240,122,88,39,192,146,58,25,184,207,94,104,103,190,145,107,148,185,1,0,0,68,0,16,0,0,0,0,0,2,0,0,160,128,64,8,0,0,0,8,64,0,0,0,0,52,64,0,16,0,129,0,0,0,0,65,0,4,0,136,0,0,0,0,0,0,0,0,0,0,0,0,4,0,0,0,10,32,64,0,0,32,32,0,20,0,128,32,0,0,1,0,4,0,0,40,1,0,0,16,1,32,0,0,16,0,64,32,0,0,0,0,0,0,0,128,16,0,0,0,131,0,64,0,0,32,64,0,0,0,8,6,0,0,0,0,0,8,0,0,0,2,16,16,4,0,40,80,8,132,0,64,0,128,64,0,65,0,0,0,0,0,64,16,1,0,36,0,0,129,0,9,64,0,0,0,0,6,0,0,2,0,1,0,0,0,128,0,16,0,8,0,128,0,1,6,0,128,128,4,0,8,0,1,0,16,10,1,0,0,0,16,0,0,0,2,0,0,4,0,0,64,1,0,0,2,0,0,0,2,0,64,0,8,0,16,0,0,1,4,2,0,32,64,81,16,0,24,0,0,8,0,144,0,0,64,8,16,0,8,0,2,32,0,0,64,128,0,16,8,136,0,2,0,0,0,132,24,139,229,22,131,149,69,210,131,122,18,0,131,38,221,21,132,96,66,160,230,153,216,131,1,9,10,132,103,101,116,104,136,103,111,49,46,49,51,46,51,133,108,105,110,117,120,160,39,207,6,45,187,127,3,47,8,180,41,100,202,29,13,201,84,59,161,13,186,184,64,59,16,6,104,128,119,137,23,223,136,39,8,135,193,134,128,177,179],"proof":[[248,113,160,89,232,21,229,118,139,147,190,61,192,149,82,65,92,124,231,242,144,39,70,87,126,160,208,38,218,92,45,17,76,149,19,160,247,117,83,108,74,228,229,64,246,232,113,17,33,68,209,141,77,116,143,134,74,195,7,126,45,242,217,177,29,153,77,25,128,128,128,128,128,128,160,9,222,167,201,202,46,111,46,237,72,14,252,141,153,239,228,28,172,236,75,178,183,47,165,225,84,179,244,219,55,11,125,128,128,128,128,128,128,128,128],[249,1,241,128,160,223,193,3,254,244,206,120,156,54,88,76,198,72,234,234,61,118,221,224,225,63,246,242,60,221,11,192,98,102,190,253,43,160,84,11,3,67,195,97,17,49,13,104,171,32,157,63,89,232,226,221,234,78,189,22,157,36,149,234,142,249,204,144,27,74,160,237,151,63,250,228,171,55,124,229,180,2,178,167,95,167,25,218,179,202,74,68,133,112,136,161,179,246,129,219,59,154,49,160,141,71,128,160,140,86,134,172,164,9,183,147,187,234,254,194,142,57,184,15,217,45,36,84,205,195,247,209,81,17,209,51,160,216,68,61,133,209,52,6,44,200,202,216,91,13,77,229,174,203,128,183,246,59,254,124,255,84,244,89,111,204,114,192,21,160,90,98,180,251,185,255,215,29,66,197,42,93,240,125,14,152,38,90,141,255,155,47,122,86,163,197,141,156,70,226,162,117,160,236,177,235,229,71,168,177,20,224,219,166,253,188,78,213,189,9,248,181,81,187,242,173,41,12,78,233,138,28,233,151,219,160,112,115,94,52,67,97,22,112,97,38,135,177,246,177,104,121,217,71,60,38,5,241,53,114,95,188,122,32,8,157,201,151,160,115,56,0,45,157,250,125,18,125,239,108,44,15,18,128,23,253,66,37,241,147,173,183,184,254,166,254,98,218,113,163,213,160,139,116,222,47,58,237,92,252,42,142,240,149,138,171,60,97,56,134,33,200,12,80,19,221,123,74,253,55,159,160,121,47,160,13,173,135,227,165,141,59,244,142,12,198,127,19,164,37,218,251,82,177,131,89,176,46,155,142,113,226,215,39,191,47,131,160,154,7,27,250,232,119,232,97,194,201,82,78,247,98,94,23,241,159,214,64,87,248,21,167,30,155,131,160,105,197,26,43,160,233,61,34,140,39,167,210,39,50,140,219,187,117,198,98,106,17,188,49,160,141,68,95,252,112,118,219,206,142,104,175,5,160,40,47,188,228,166,39,128,177,241,44,2,180,84,178,35,45,76,9,67,167,70,226,192,138,185,170,205,110,190,6,163,68,160,88,211,112,220,92,97,52,179,239,5,189,65,220,39,140,221,38,173,108,53,42,206,5,89,139,96,134,151,77,222,96,67,128],[249,2,14,32,185,2,10,249,2,7,1,131,4,23,235,185,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,16,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,248,253,248,251,148,185,247,33,158,67,78,170,112,33,174,95,158,205,12,171,194,64,84,71,163,248,66,160,91,253,175,236,57,174,146,96,226,220,66,250,35,21,1,244,101,251,175,87,166,187,188,197,23,157,14,86,105,51,218,174,160,0,0,0,0,0,0,0,0,0,0,0,0,137,27,39,73,35,139,39,255,88,233,81,8,142,85,176,77,231,29,195,116,184,160,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,96,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,197,18,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,194,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,114,111,111,116,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]],"skip_bridge_call":false}"#;
const PROOF_DATA_ETH: &'static str = r#"{"log_index":0,"log_entry_data":[248,188,148,101,151,223,196,35,161,116,183,108,156,254,237,145,7,46,56,130,95,42,232,248,99,160,109,186,216,63,74,73,240,97,147,138,76,227,92,213,206,167,97,83,8,64,17,0,208,248,106,126,133,216,214,202,191,21,160,0,0,0,0,0,0,0,0,0,0,0,0,137,27,39,73,35,139,39,255,88,233,81,8,142,85,176,77,231,29,195,116,160,0,0,0,0,0,0,0,0,0,0,0,0,137,27,39,73,35,139,39,255,88,233,81,8,142,85,176,77,231,29,195,116,184,64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,12,54,144,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,144],"receipt_index":6,"receipt_data":[249,1,200,1,131,26,97,185,185,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,8,0,0,0,0,0,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,248,190,248,188,148,101,151,223,196,35,161,116,183,108,156,254,237,145,7,46,56,130,95,42,232,248,99,160,109,186,216,63,74,73,240,97,147,138,76,227,92,213,206,167,97,83,8,64,17,0,208,248,106,126,133,216,214,202,191,21,160,0,0,0,0,0,0,0,0,0,0,0,0,137,27,39,73,35,139,39,255,88,233,81,8,142,85,176,77,231,29,195,116,160,0,0,0,0,0,0,0,0,0,0,0,0,137,27,39,73,35,139,39,255,88,233,81,8,142,85,176,77,231,29,195,116,184,64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,12,54,144,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,144],"header_data":[249,2,21,160,161,193,231,252,32,76,15,59,111,172,246,181,99,116,162,240,31,222,83,91,200,239,11,93,197,149,150,217,219,79,47,104,160,29,204,77,232,222,199,93,122,171,133,181,103,182,204,212,26,211,18,69,27,148,138,116,19,240,161,66,253,64,212,147,71,148,3,62,246,219,159,189,14,230,14,41,49,144,107,152,127,224,40,4,113,160,160,3,183,138,112,223,193,238,76,58,34,39,5,48,219,126,90,242,8,254,128,241,233,30,137,224,121,36,140,176,152,137,103,160,75,232,95,80,188,8,102,2,76,12,106,69,142,199,233,74,125,30,90,1,213,211,141,100,68,22,134,123,192,183,213,67,160,193,27,74,244,141,199,119,106,16,70,140,25,46,9,173,240,59,17,218,215,220,11,78,153,182,41,7,159,130,194,108,34,185,1,0,52,165,65,1,8,26,0,8,148,32,128,0,177,8,64,129,76,128,4,0,72,19,9,33,0,131,32,24,74,80,129,0,147,67,100,17,0,17,112,0,0,208,0,0,160,144,8,2,1,0,0,40,136,64,0,34,68,146,4,128,8,38,18,69,4,96,80,0,12,48,148,24,0,0,10,72,136,0,22,224,24,131,32,32,104,90,2,8,1,0,169,9,128,132,34,5,232,33,65,10,6,1,113,202,68,24,0,128,0,3,40,8,16,0,169,16,1,8,206,66,32,8,90,144,13,192,80,1,0,0,136,0,9,2,0,0,97,100,64,130,64,4,0,16,132,16,32,33,8,0,4,72,4,129,28,120,66,0,0,32,2,66,181,0,192,169,0,0,0,0,5,3,32,2,4,66,0,0,17,144,0,14,0,2,6,6,144,162,128,0,139,0,97,83,33,10,0,128,80,24,0,5,0,58,2,99,0,1,48,22,66,72,1,112,65,152,1,65,1,146,1,40,164,128,129,16,130,4,4,81,6,3,6,64,4,1,4,1,128,72,0,8,36,128,129,4,0,64,132,0,24,8,144,8,33,0,132,28,149,247,230,131,153,94,102,131,122,18,0,131,121,228,27,132,96,121,135,63,151,214,131,1,10,1,132,103,101,116,104,134,103,111,49,46,49,54,133,108,105,110,117,120,160,211,62,149,208,144,11,49,49,244,81,132,152,40,108,13,205,228,207,189,220,243,10,93,35,118,28,238,243,8,10,31,79,136,56,35,86,22,39,212,182,221],"proof":[[249,1,49,160,40,52,61,93,249,149,77,228,91,47,138,204,184,83,99,163,218,2,58,226,7,193,222,36,174,10,96,58,64,141,177,16,160,194,20,100,120,187,107,207,143,189,239,126,2,98,125,113,233,130,25,189,36,33,19,116,2,227,77,155,121,164,224,158,99,160,102,95,235,60,77,191,204,127,156,81,112,169,6,91,228,140,78,248,185,134,200,229,187,24,177,158,50,27,108,174,190,215,160,165,183,27,22,130,131,193,127,245,78,128,36,141,194,160,77,148,192,32,180,196,96,214,125,134,28,123,74,184,133,75,178,160,200,242,112,211,168,64,222,3,34,101,92,5,157,101,236,252,101,166,105,160,107,4,103,183,51,59,161,140,45,126,162,72,160,176,7,132,99,183,135,252,15,108,26,127,255,244,123,144,182,149,139,19,221,66,70,243,58,78,47,200,240,39,117,237,165,160,117,255,193,226,106,214,43,41,134,223,139,8,91,129,214,251,25,235,51,107,127,158,211,26,138,132,231,160,13,7,23,217,160,99,66,167,41,62,92,113,220,248,227,176,100,243,32,138,127,164,188,248,98,168,76,112,81,33,144,8,173,87,140,182,148,160,192,68,173,9,93,34,73,147,145,182,166,209,62,181,112,236,28,27,242,99,121,181,72,120,169,48,127,36,12,178,202,139,128,128,128,128,128,128,128,128],[249,1,241,128,160,30,119,18,4,248,188,176,96,146,48,75,22,195,107,76,238,26,216,63,216,198,244,148,161,33,43,70,45,212,5,81,156,160,34,245,53,221,177,39,229,1,26,110,100,85,102,171,72,49,118,158,14,34,13,13,149,189,154,210,39,8,181,249,77,5,160,30,253,233,173,196,224,27,183,31,54,54,63,136,2,226,220,178,78,56,161,21,182,122,104,113,250,199,3,153,101,175,223,160,159,241,18,1,202,43,48,190,84,192,252,191,238,74,213,161,236,61,40,168,90,212,39,124,53,216,80,220,131,113,241,69,160,252,57,54,60,94,218,194,31,163,224,111,253,17,33,180,77,168,175,73,66,233,142,135,189,131,30,198,195,142,111,138,174,160,140,70,132,11,8,139,100,35,178,194,116,149,86,237,150,17,213,165,174,194,253,60,226,188,106,243,123,103,108,189,244,100,160,101,205,74,36,174,110,51,102,0,31,194,99,174,188,19,4,231,81,47,87,179,197,159,240,19,112,176,132,248,221,146,213,160,178,96,70,80,21,16,69,137,236,102,133,196,69,59,246,187,255,24,101,30,222,247,235,210,113,126,178,28,215,1,182,138,160,114,105,212,192,214,243,173,197,90,103,131,93,113,140,250,59,35,28,241,236,154,49,94,230,194,245,45,183,204,251,69,43,160,102,135,250,63,213,103,104,232,23,143,144,169,168,20,240,201,209,101,250,210,220,190,124,171,1,231,2,204,30,253,89,251,160,36,6,72,221,22,240,240,7,79,213,82,39,240,172,95,197,227,175,113,99,139,224,24,43,162,94,91,36,17,80,207,220,160,169,205,200,89,82,76,235,78,167,158,181,248,224,73,68,252,42,175,210,210,174,76,168,8,97,122,182,30,249,198,75,87,160,180,152,73,83,196,162,242,227,112,247,177,68,121,240,146,19,217,166,68,252,53,103,87,199,9,117,80,173,142,171,229,247,160,49,105,44,32,67,209,210,63,87,212,96,82,74,115,152,85,18,139,237,55,138,1,7,160,12,60,91,125,192,183,236,249,160,125,165,55,119,71,188,255,109,25,163,228,212,187,172,52,164,244,46,157,165,67,205,254,99,82,210,41,91,194,145,158,46,128],[249,1,207,32,185,1,203,249,1,200,1,131,26,97,185,185,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,8,0,0,0,0,0,0,0,0,0,0,0,64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,248,190,248,188,148,101,151,223,196,35,161,116,183,108,156,254,237,145,7,46,56,130,95,42,232,248,99,160,109,186,216,63,74,73,240,97,147,138,76,227,92,213,206,167,97,83,8,64,17,0,208,248,106,126,133,216,214,202,191,21,160,0,0,0,0,0,0,0,0,0,0,0,0,137,27,39,73,35,139,39,255,88,233,81,8,142,85,176,77,231,29,195,116,160,0,0,0,0,0,0,0,0,0,0,0,0,137,27,39,73,35,139,39,255,88,233,81,8,142,85,176,77,231,29,195,116,184,64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,12,54,144,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,144]],"skip_bridge_call":false}"#;
const DEPOSITED_RECIPIENT: &'static str = "root";
const PROVER_ACCOUNT: &'static str = "eth_connector.root";
const CUSTODIAN_ADDRESS: &'static str = "b9f7219e434EAA7021Ae5f9Ecd0CaBc2405447A3";
const DEPOSITED_AMOUNT: u128 = 50450;
const DEPOSITED_FEE: u128 = 450;
const RECIPIENT_ETH_ADDRESS: &'static str = "891b2749238b27ff58e951088e55b04de71dc374";
const EVM_CUSTODIAN_ADDRESS: &'static str = "6597dfc423a174b76c9cfeed91072e38825f2ae8";
const DEPOSITED_EVM_AMOUNT: u128 = 800400;
const DEPOSITED_EVM_FEE: u128 = 400;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    EVM_WASM_BYTES => "release.wasm"
}

fn init(custodian_address: &str) -> (UserAccount, UserAccount) {
    let master_account = near_sdk_sim::init_simulator(None);
    let contract_account = master_account.deploy(
        *EVM_WASM_BYTES,
        CONTRACT_ACC.to_string(),
        to_yocto("1000000"),
    );
    contract_account
        .call(
            CONTRACT_ACC.to_string(),
            "new",
            &NewCallArgs {
                chain_id: [0u8; 32],
                owner_id: master_account.account_id.clone(),
                bridge_prover_id: accounts(0).to_string(),
                upgrade_delay_blocks: 1,
            }
            .try_to_vec()
            .unwrap(),
            DEFAULT_GAS,
            STORAGE_AMOUNT,
        )
        .assert_success();
    master_account
        .call(
            CONTRACT_ACC.to_string(),
            "new_eth_connector",
            json!({
                "prover_account": PROVER_ACCOUNT,
                "eth_custodian_address": custodian_address,
            })
            .to_string()
            .as_bytes(),
            DEFAULT_GAS,
            0,
        )
        .assert_success();
    (master_account, contract_account)
}

fn validate_eth_address(address: &str) -> EthAddress {
    let data = hex::decode(address).unwrap();
    assert_eq!(data.len(), 20);
    let mut result = [0u8; 20];
    result.copy_from_slice(&data);
    result
}

fn call_deposit_near(master_account: &UserAccount) {
    let res = master_account.call(
        CONTRACT_ACC.to_string(),
        "deposit_near",
        PROOF_DATA_NEAR.to_string().as_bytes(),
        DEFAULT_GAS,
        0,
    );
    res.assert_success();
}

fn call_deposit_eth(master_account: &UserAccount) {
    let res = master_account.call(
        CONTRACT_ACC.to_string(),
        "deposit_eth",
        PROOF_DATA_ETH.to_string().as_bytes(),
        DEFAULT_GAS,
        0,
    );
    res.assert_success();
    //println!("#1: {:#?}", res.logs());
}

fn get_near_balance(master_account: &UserAccount, acc: &str) -> u128 {
    let balance = master_account.view(
        CONTRACT_ACC.to_string(),
        "ft_balance_of",
        json!({ "account_id": acc }).to_string().as_bytes(),
    );
    String::from_utf8(balance.unwrap())
        .unwrap()
        .parse()
        .unwrap()
}

fn get_eth_balance(master_account: &UserAccount, address: EthAddress) -> u128 {
    let address = hex::encode(address);
    let balance = master_account.view(
        CONTRACT_ACC.to_string(),
        "ft_balance_of_eth",
        json!({ "address": address }).to_string().as_bytes(),
    );
    String::from_utf8(balance.unwrap())
        .unwrap()
        .parse()
        .unwrap()
}

fn total_supply(master_account: &UserAccount) -> u128 {
    let balance = master_account.view(CONTRACT_ACC.to_string(), "ft_total_supply", &[]);
    String::from_utf8(balance.unwrap())
        .unwrap()
        .parse()
        .unwrap()
}

fn total_supply_near(master_account: &UserAccount) -> u128 {
    let balance = master_account.view(CONTRACT_ACC.to_string(), "ft_total_supply_near", &[]);
    String::from_utf8(balance.unwrap())
        .unwrap()
        .parse()
        .unwrap()
}

fn total_supply_eth(master_account: &UserAccount) -> u128 {
    let balance = master_account.view(CONTRACT_ACC.to_string(), "ft_total_supply_eth", &[]);
    String::from_utf8(balance.unwrap())
        .unwrap()
        .parse()
        .unwrap()
}

#[test]
fn test_near_deposit_balance_total_supply() {
    let (master_account, _contract) = init(CUSTODIAN_ADDRESS);
    call_deposit_near(&master_account);

    let balance = get_near_balance(&master_account, DEPOSITED_RECIPIENT);
    assert_eq!(balance, DEPOSITED_AMOUNT - DEPOSITED_FEE);

    let balance = get_near_balance(&master_account, CONTRACT_ACC);
    assert_eq!(balance, DEPOSITED_FEE);

    let balance = total_supply(&master_account);
    assert_eq!(balance, DEPOSITED_AMOUNT);

    let balance = total_supply_near(&master_account);
    assert_eq!(balance, DEPOSITED_AMOUNT);

    let balance = total_supply_eth(&master_account);
    assert_eq!(balance, 0);
}

#[test]
fn test_deposit_eth_and_near() {
    let (master_account, _contract) = init(CUSTODIAN_ADDRESS);
    call_deposit_near(&master_account);
    call_deposit_eth(&master_account);
}

#[test]
fn test_eth_deposit_balance_total_supply() {
    let (master_account, _contract) = init(EVM_CUSTODIAN_ADDRESS);
    call_deposit_eth(&master_account);

    let balance = get_eth_balance(
        &master_account,
        validate_eth_address("891b2749238b27ff58e951088e55b04de71dc374"),
    );
    assert_eq!(balance, DEPOSITED_EVM_AMOUNT - DEPOSITED_EVM_FEE);

    let balance = total_supply(&master_account);
    assert_eq!(balance, DEPOSITED_AMOUNT);
}

#[test]
fn test_withdraw_near() {
    let (master_account, _contract) = init(CUSTODIAN_ADDRESS);
    call_deposit_near(&master_account);

    let withdraw_amount = 100;
    let res = master_account.call(
        CONTRACT_ACC.to_string(),
        "withdraw_near",
        json!({
            "recipient_id": RECIPIENT_ETH_ADDRESS,
            "amount": withdraw_amount,
        })
        .to_string()
        .as_bytes(),
        DEFAULT_GAS,
        0,
    );
    res.assert_success();

    let balance = get_near_balance(&master_account, DEPOSITED_RECIPIENT);
    assert_eq!(
        balance,
        DEPOSITED_AMOUNT - DEPOSITED_FEE - withdraw_amount as u128
    );

    let balance = get_near_balance(&master_account, CONTRACT_ACC);
    assert_eq!(balance, DEPOSITED_FEE);

    let balance = total_supply(&master_account);
    assert_eq!(balance, DEPOSITED_AMOUNT);
}

#[test]
fn test_withdraw_eth() {
    let (master_account, _contract_account) = init(CUSTODIAN_ADDRESS);
    let res = master_account
        .call(
            CONTRACT_ACC.to_string(),
            "withdraw_eth",
            json!({
                "sender": "891B2749238B27fF58e951088e55b04de71Dc374", 
                "eth_recipient": "891B2749238B27fF58e951088e55b04de71Dc374", 
                "amount": "7654321",
                "eip712_signature": "51ea7c8a54da3ffc1f6af82f9e535e156577583583d3e9de375139b41443ab5f4bddc25f69134a2d0fba2aa701da1532a94a013dd811d6c7edbbe94542a62ba41c"
            }).to_string().as_bytes(),
            DEFAULT_GAS,
            0,
        );
    res.assert_success();
    for s in res.logs().iter() {
        println!("[log] {}", s);
    }
}

#[test]
fn test_ft_transfer() {
    let (master_account, _contract) = init(CUSTODIAN_ADDRESS);
    call_deposit_near(&master_account);

    let transfer_amount = 777;
    let res = master_account.call(
        CONTRACT_ACC.to_string(),
        "ft_transfer",
        json!({
            "receiver_id": CONTRACT_ACC,
            "amount": transfer_amount,
            "memo": "transfer memo"
        })
        .to_string()
        .as_bytes(),
        DEFAULT_GAS,
        1,
    );
    res.assert_success();

    let balance = get_near_balance(&master_account, DEPOSITED_RECIPIENT);
    assert_eq!(
        balance,
        DEPOSITED_AMOUNT - DEPOSITED_FEE - transfer_amount as u128
    );

    let balance = get_near_balance(&master_account, CONTRACT_ACC);
    assert_eq!(balance, DEPOSITED_FEE + transfer_amount as u128);
}

#[test]
fn test_ft_transfer_call() {
    let (master_account, _contract) = init(CUSTODIAN_ADDRESS);
    call_deposit_near(&master_account);

    let balance = get_near_balance(&master_account, DEPOSITED_RECIPIENT);
    assert_eq!(balance, DEPOSITED_AMOUNT - DEPOSITED_FEE);

    let balance = get_near_balance(&master_account, CONTRACT_ACC);
    assert_eq!(balance, DEPOSITED_FEE);

    let transfer_amount = 100;
    let res = master_account.call(
        CONTRACT_ACC.to_string(),
        "ft_transfer_call",
        json!({
            "receiver_id": CONTRACT_ACC,
            "amount": transfer_amount,
            "memo": "transfer memo",
            "msg": "some message"
        })
        .to_string()
        .as_bytes(),
        DEFAULT_GAS,
        1,
    );
    res.assert_success();

    let balance = get_near_balance(&master_account, DEPOSITED_RECIPIENT);
    assert_eq!(
        balance,
        DEPOSITED_AMOUNT - DEPOSITED_FEE - transfer_amount as u128
    );

    let balance = get_near_balance(&master_account, CONTRACT_ACC);
    assert_eq!(balance, DEPOSITED_FEE + transfer_amount as u128);
}
