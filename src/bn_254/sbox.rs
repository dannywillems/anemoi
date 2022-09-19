use super::BigInteger256;
use super::Felt;

use ark_ff::Field;

#[allow(unused)]
/// Exponent of the Anemoi S-Box
pub(crate) const ALPHA: u32 = 5;

#[allow(unused)]
/// Inverse exponent
pub(crate) const INV_ALPHA: [u64; 4] = [
    0x180d04d5f031fee9,
    0xd633c43a29c71dd2,
    0x49b9b57c33cd568b,
    0x135b52945a13d9aa,
];

/// Multiplier of the Anemoi S-Box
#[allow(unused)]
pub(crate) const BETA: u32 = 3;

/// First added constant of the Anemoi S-Box
pub(crate) const DELTA: Felt = Felt::new(BigInteger256([
    0xafd49a8c34aeae4c,
    0xe0a8c73e1f684743,
    0xb4ea4db753538a2d,
    0x14cf9766d3bdd51d,
]));

#[allow(unused)]
/// Second added constant of the Anemoi S-Box
pub(crate) const QUAD: u32 = 2;

#[inline(always)]
pub(crate) fn exp_inv_alpha(x: &Felt) -> Felt {
    let t14 = x.square(); //      1: 2
    let t2 = t14 * x; //          2: 3
    let t1 = t14.square(); //     3: 4
    let t0 = t2 * t14; //         4: 5
    let t2 = t2.square(); //      5: 6
    let t13 = t1.square(); //     6: 8
    let t1 = t0 * t1; //          7: 9
    let t6 = t13 * t0; //         8: 13
    let t0 = t13 * t2; //         9: 14
    let t12 = t13.square(); //   10: 16
    let t8 = t6 * t1; //         11: 22
    let t3 = t6.square(); //     12: 26
    let t15 = t12 * t0; //       13: 30
    let t4 = t8 * t1; //         14: 31
    let t9 = t3 * t12; //        15: 42
    let t20 = t4 * t0; //        16: 45
    let t3 = t4 * t8; //         17: 53
    let t5 = t4.square(); //     18: 62
    let t7 = t20 * t8; //        19: 67
    let t10 = t3 * t12; //       20: 69
    let t18 = t9 * t4; //        21: 73
    let t3 = t10 * t6; //        22: 82
    let t11 = t18 * t5; //       23: 135
    let t21 = t11 * t15; //      24: 165
    let t16 = t21 * t12; //      25: 181
    let t12 = t21 * t9; //       26: 207
    let t14 = t12 * t14; //      27: 209
    let t15 = t12 * t2; //       28: 213
    let t2 = t21 * t3; //        29: 247
    let t5 = t2 * t5; //         30: 309
    let t19 = t5 * t13; //       31: 317
    let t3 = t19 * t3; //        32: 399
    let t17 = t3 * t9; //        33: 441
    let t9 = t17 * t0; //        34: 455
    let t8 = t9 * t8; //         35: 477
    let t13 = t8 * t0; //        36: 491
    let mut t0 = t5.square(); // 37: 618
    t0 *= t0; //                 38: 1236
    t0 *= t0; //                 39: 2472
    t0 *= t0; //                 40: 4944
    t0 *= t0; //                 41: 9888
    t0 *= t0; //                 42: 19776
    t0 *= t0; //                 43: 39552
    t0 *= t0; //                 44: 79104
    t0 *= t16; //                45: 79285
    t0 *= t0; //                 46: 158570
    t0 *= t0; //                 47: 317140
    t0 *= t0; //                 48: 634280
    t0 *= t0; //                 49: 1268560
    t0 *= t0; //                 50: 2537120
    t0 *= t0; //                 51: 5074240
    t0 *= t0; //                 52: 10148480
    t0 *= t0; //                 53: 20296960
    t0 *= t0; //                 54: 40593920
    t0 *= t0; //                 55: 81187840
    t0 *= t21; //                56: 81188005
    t0 *= t0; //                 57: 162376010
    t0 *= t0; //                 58: 324752020
    t0 *= t0; //                 59: 649504040
    t0 *= t0; //                 60: 1299008080
    t0 *= t0; //                 61: 2598016160
    t0 *= t0; //                 62: 5196032320
    t0 *= t0; //                 63: 10392064640
    t0 *= t0; //                 64: 20784129280
    t0 *= t0; //                 65: 41568258560
    t0 *= t20; //                66: 41568258605
    t0 *= t0; //                 67: 83136517210
    t0 *= t0; //                 68: 166273034420
    t0 *= t0; //                 69: 332546068840
    t0 *= t0; //                 70: 665092137680
    t0 *= t0; //                 71: 1330184275360
    t0 *= t0; //                 72: 2660368550720
    t0 *= t0; //                 73: 5320737101440
    t0 *= t0; //                 74: 10641474202880
    t0 *= t0; //                 75: 21282948405760
    t0 *= t0; //                 76: 42565896811520
    t0 *= t0; //                 77: 85131793623040
    t0 *= t0; //                 78: 170263587246080
    t0 *= t0; //                 79: 340527174492160
    t0 *= t19; //                80: 340527174492477
    t0 *= t0; //                 81: 681054348984954
    t0 *= t0; //                 82: 1362108697969908
    t0 *= t0; //                 83: 2724217395939816
    t0 *= t0; //                 84: 5448434791879632
    t0 *= t0; //                 85: 10896869583759264
    t0 *= t0; //                 86: 21793739167518528
    t0 *= t0; //                 87: 43587478335037056
    t0 *= t0; //                 88: 87174956670074112
    t0 *= t0; //                 89: 174349913340148224
    t0 *= t5; //                 90: 174349913340148533
    t0 *= t0; //                 91: 348699826680297066
    t0 *= t0; //                 92: 697399653360594132
    t0 *= t0; //                 93: 1394799306721188264
    t0 *= t0; //                 94: 2789598613442376528
    t0 *= t0; //                 95: 5579197226884753056
    t0 *= t0; //                 96: 11158394453769506112
    t0 *= t0; //                 97: 22316788907539012224
    t0 *= t0; //                 98: 44633577815078024448
    t0 *= t18; //                99: 44633577815078024521
    t0 *= t0; //                100: 89267155630156049042
    t0 *= t0; //                101: 178534311260312098084
    t0 *= t0; //                102: 357068622520624196168
    t0 *= t0; //                103: 714137245041248392336
    t0 *= t0; //                104: 1428274490082496784672
    t0 *= t0; //                105: 2856548980164993569344
    t0 *= t0; //                106: 5713097960329987138688
    t0 *= t0; //                107: 11426195920659974277376
    t0 *= t0; //                108: 22852391841319948554752
    t0 *= t0; //                109: 45704783682639897109504
    t0 *= t0; //                110: 91409567365279794219008
    t0 *= t17; //               111: 91409567365279794219449
    t0 *= t0; //                112: 182819134730559588438898
    t0 *= t0; //                113: 365638269461119176877796
    t0 *= t0; //                114: 731276538922238353755592
    t0 *= t0; //                115: 1462553077844476707511184
    t0 *= t0; //                116: 2925106155688953415022368
    t0 *= t0; //                117: 5850212311377906830044736
    t0 *= t0; //                118: 11700424622755813660089472
    t0 *= t0; //                119: 23400849245511627320178944
    t0 *= t16; //               120: 23400849245511627320179125
    t0 *= t0; //                121: 46801698491023254640358250
    t0 *= t0; //                122: 93603396982046509280716500
    t0 *= t0; //                123: 187206793964093018561433000
    t0 *= t0; //                124: 374413587928186037122866000
    t0 *= t0; //                125: 748827175856372074245732000
    t0 *= t0; //                126: 1497654351712744148491464000
    t0 *= t4; //                127: 1497654351712744148491464031
    t0 *= t0; //                128: 2995308703425488296982928062
    t0 *= t0; //                129: 5990617406850976593965856124
    t0 *= t0; //                130: 11981234813701953187931712248
    t0 *= t0; //                131: 23962469627403906375863424496
    t0 *= t0; //                132: 47924939254807812751726848992
    t0 *= t0; //                133: 95849878509615625503453697984
    t0 *= t0; //                134: 191699757019231251006907395968
    t0 *= t0; //                135: 383399514038462502013814791936
    t0 *= t0; //                136: 766799028076925004027629583872
    t0 *= t0; //                137: 1533598056153850008055259167744
    t0 *= t0; //                138: 3067196112307700016110518335488
    t0 *= t0; //                139: 6134392224615400032221036670976
    t0 *= t12; //               140: 6134392224615400032221036671183
    t0 *= t0; //                141: 12268784449230800064442073342366
    t0 *= t0; //                142: 24537568898461600128884146684732
    t0 *= t0; //                143: 49075137796923200257768293369464
    t0 *= t0; //                144: 98150275593846400515536586738928
    t0 *= t0; //                145: 196300551187692801031073173477856
    t0 *= t0; //                146: 392601102375385602062146346955712
    t0 *= t0; //                147: 785202204750771204124292693911424
    t0 *= t0; //                148: 1570404409501542408248585387822848
    t0 *= t0; //                149: 3140808819003084816497170775645696
    t0 *= t0; //                150: 6281617638006169632994341551291392
    t0 *= t15; //               151: 6281617638006169632994341551291605
    t0 *= t0; //                152: 12563235276012339265988683102583210
    t0 *= t0; //                153: 25126470552024678531977366205166420
    t0 *= t0; //                154: 50252941104049357063954732410332840
    t0 *= t0; //                155: 100505882208098714127909464820665680
    t0 *= t0; //                156: 201011764416197428255818929641331360
    t0 *= t0; //                157: 402023528832394856511637859282662720
    t0 *= t0; //                158: 804047057664789713023275718565325440
    t0 *= t0; //                159: 1608094115329579426046551437130650880
    t0 *= t0; //                160: 3216188230659158852093102874261301760
    t0 *= t14; //               161: 3216188230659158852093102874261301969
    t0 *= t0; //                162: 6432376461318317704186205748522603938
    t0 *= t0; //                163: 12864752922636635408372411497045207876
    t0 *= t0; //                164: 25729505845273270816744822994090415752
    t0 *= t0; //                165: 51459011690546541633489645988180831504
    t0 *= t0; //                166: 102918023381093083266979291976361663008
    t0 *= t0; //                167: 205836046762186166533958583952723326016
    t0 *= t0; //                168: 411672093524372333067917167905446652032
    t0 *= t0; //                169: 823344187048744666135834335810893304064
    t0 *= t0; //                170: 1646688374097489332271668671621786608128
    t0 *= t0; //                171: 3293376748194978664543337343243573216256
    t0 *= t13; //               172: 3293376748194978664543337343243573216747
    t0 *= t0; //                173: 6586753496389957329086674686487146433494
    t0 *= t0; //                174: 13173506992779914658173349372974292866988
    t0 *= t0; //                175: 26347013985559829316346698745948585733976
    t0 *= t0; //                176: 52694027971119658632693397491897171467952
    t0 *= t0; //                177: 105388055942239317265386794983794342935904
    t0 *= t0; //                178: 210776111884478634530773589967588685871808
    t0 *= t0; //                179: 421552223768957269061547179935177371743616
    t0 *= t0; //                180: 843104447537914538123094359870354743487232
    t0 *= t0; //                181: 1686208895075829076246188719740709486974464
    t0 *= t0; //                182: 3372417790151658152492377439481418973948928
    t0 *= t0; //                183: 6744835580303316304984754878962837947897856
    t0 *= t12; //               184: 6744835580303316304984754878962837947898063
    t0 *= t0; //                185: 13489671160606632609969509757925675895796126
    t0 *= t0; //                186: 26979342321213265219939019515851351791592252
    t0 *= t0; //                187: 53958684642426530439878039031702703583184504
    t0 *= t0; //                188: 107917369284853060879756078063405407166369008
    t0 *= t0; //                189: 215834738569706121759512156126810814332738016
    t0 *= t0; //                190: 431669477139412243519024312253621628665476032
    t0 *= t0; //                191: 863338954278824487038048624507243257330952064
    t0 *= t0; //                192: 1726677908557648974076097249014486514661904128
    t0 *= t0; //                193: 3453355817115297948152194498028973029323808256
    t0 *= t0; //                194: 6906711634230595896304388996057946058647616512
    t0 *= t0; //                195: 13813423268461191792608777992115892117295233024
    t0 *= t11; //               196: 13813423268461191792608777992115892117295233159
    t0 *= t0; //                197: 27626846536922383585217555984231784234590466318
    t0 *= t0; //                198: 55253693073844767170435111968463568469180932636
    t0 *= t0; //                199: 110507386147689534340870223936927136938361865272
    t0 *= t0; //                200: 221014772295379068681740447873854273876723730544
    t0 *= t0; //                201: 442029544590758137363480895747708547753447461088
    t0 *= t0; //                202: 884059089181516274726961791495417095506894922176
    t0 *= t0; //                203: 1768118178363032549453923582990834191013789844352
    t0 *= t0; //                204: 3536236356726065098907847165981668382027579688704
    t0 *= t10; //               205: 3536236356726065098907847165981668382027579688773
    t0 *= t0; //                206: 7072472713452130197815694331963336764055159377546
    t0 *= t0; //                207: 14144945426904260395631388663926673528110318755092
    t0 *= t0; //                208: 28289890853808520791262777327853347056220637510184
    t0 *= t0; //                209: 56579781707617041582525554655706694112441275020368
    t0 *= t0; //                210: 113159563415234083165051109311413388224882550040736
    t0 *= t0; //                211: 226319126830468166330102218622826776449765100081472
    t0 *= t0; //                212: 452638253660936332660204437245653552899530200162944
    t0 *= t0; //                213: 905276507321872665320408874491307105799060400325888
    t0 *= t0; //                214: 1810553014643745330640817748982614211598120800651776
    t0 *= t0; //                215: 3621106029287490661281635497965228423196241601303552
    t0 *= t0; //                216: 7242212058574981322563270995930456846392483202607104
    t0 *= t9; //                217: 7242212058574981322563270995930456846392483202607559
    t0 *= t0; //                218: 14484424117149962645126541991860913692784966405215118
    t0 *= t0; //                219: 28968848234299925290253083983721827385569932810430236
    t0 *= t0; //                220: 57937696468599850580506167967443654771139865620860472
    t0 *= t0; //                221: 115875392937199701161012335934887309542279731241720944
    t0 *= t0; //                222: 231750785874399402322024671869774619084559462483441888
    t0 *= t0; //                223: 463501571748798804644049343739549238169118924966883776
    t0 *= t0; //                224: 927003143497597609288098687479098476338237849933767552
    t0 *= t0; //                225: 1854006286995195218576197374958196952676475699867535104
    t0 *= t0; //                226: 3708012573990390437152394749916393905352951399735070208
    t0 *= t0; //                227: 7416025147980780874304789499832787810705902799470140416
    t0 *= t0; //                228: 14832050295961561748609578999665575621411805598940280832
    t0 *= t0; //                229: 29664100591923123497219157999331151242823611197880561664
    t0 *= t8; //                230: 29664100591923123497219157999331151242823611197880562141
    t0 *= t0; //                231: 59328201183846246994438315998662302485647222395761124282
    t0 *= t0; //                232: 118656402367692493988876631997324604971294444791522248564
    t0 *= t0; //                233: 237312804735384987977753263994649209942588889583044497128
    t0 *= t0; //                234: 474625609470769975955506527989298419885177779166088994256
    t0 *= t0; //                235: 949251218941539951911013055978596839770355558332177988512
    t0 *= t0; //                236: 1898502437883079903822026111957193679540711116664355977024
    t0 *= t0; //                237: 3797004875766159807644052223914387359081422233328711954048
    t0 *= t0; //                238: 7594009751532319615288104447828774718162844466657423908096
    t0 *= t0; //                239: 15188019503064639230576208895657549436325688933314847816192
    t0 *= t7; //                240: 15188019503064639230576208895657549436325688933314847816259
    t0 *= t0; //                241: 30376039006129278461152417791315098872651377866629695632518
    t0 *= t0; //                242: 60752078012258556922304835582630197745302755733259391265036
    t0 *= t0; //                243: 121504156024517113844609671165260395490605511466518782530072
    t0 *= t0; //                244: 243008312049034227689219342330520790981211022933037565060144
    t0 *= t0; //                245: 486016624098068455378438684661041581962422045866075130120288
    t0 *= t0; //                246: 972033248196136910756877369322083163924844091732150260240576
    t0 *= t0; //                247: 1944066496392273821513754738644166327849688183464300520481152
    t0 *= t0; //                248: 3888132992784547643027509477288332655699376366928601040962304
    t0 *= t0; //                249: 7776265985569095286055018954576665311398752733857202081924608
    t0 *= t0; //                250: 15552531971138190572110037909153330622797505467714404163849216
    t0 *= t0; //                251: 31105063942276381144220075818306661245595010935428808327698432
    t0 *= t6; //                252: 31105063942276381144220075818306661245595010935428808327698445
    t0 *= t0; //                253: 62210127884552762288440151636613322491190021870857616655396890
    t0 *= t0; //                254: 124420255769105524576880303273226644982380043741715233310793780
    t0 *= t0; //                255: 248840511538211049153760606546453289964760087483430466621587560
    t0 *= t0; //                256: 497681023076422098307521213092906579929520174966860933243175120
    t0 *= t0; //                257: 995362046152844196615042426185813159859040349933721866486350240
    t0 *= t0; //                258: 1990724092305688393230084852371626319718080699867443732972700480
    t0 *= t0; //                259: 3981448184611376786460169704743252639436161399734887465945400960
    t0 *= t0; //                260: 7962896369222753572920339409486505278872322799469774931890801920
    t0 *= t0; //                261: 15925792738445507145840678818973010557744645598939549863781603840
    t0 *= t0; //                262: 31851585476891014291681357637946021115489291197879099727563207680
    t0 *= t0; //                263: 63703170953782028583362715275892042230978582395758199455126415360
    t0 *= t0; //                264: 127406341907564057166725430551784084461957164791516398910252830720
    t0 *= t0; //                265: 254812683815128114333450861103568168923914329583032797820505661440
    t0 *= t0; //                266: 509625367630256228666901722207136337847828659166065595641011322880
    t0 *= t5; //                267: 509625367630256228666901722207136337847828659166065595641011323189
    t0 *= t0; //                268: 1019250735260512457333803444414272675695657318332131191282022646378
    t0 *= t0; //                269: 2038501470521024914667606888828545351391314636664262382564045292756
    t0 *= t0; //                270: 4077002941042049829335213777657090702782629273328524765128090585512
    t0 *= t0; //                271: 8154005882084099658670427555314181405565258546657049530256181171024
    t0 *= t0; //                272: 16308011764168199317340855110628362811130517093314099060512362342048
    t0 *= t0; //                273: 32616023528336398634681710221256725622261034186628198121024724684096
    t0 *= t4; //                274: 32616023528336398634681710221256725622261034186628198121024724684127
    t0 *= t0; //                275: 65232047056672797269363420442513451244522068373256396242049449368254
    t0 *= t0; //                276: 130464094113345594538726840885026902489044136746512792484098898736508
    t0 *= t0; //                277: 260928188226691189077453681770053804978088273493025584968197797473016
    t0 *= t0; //                278: 521856376453382378154907363540107609956176546986051169936395594946032
    t0 *= t0; //                279: 1043712752906764756309814727080215219912353093972102339872791189892064
    t0 *= t0; //                280: 2087425505813529512619629454160430439824706187944204679745582379784128
    t0 *= t0; //                281: 4174851011627059025239258908320860879649412375888409359491164759568256
    t0 *= t0; //                282: 8349702023254118050478517816641721759298824751776818718982329519136512
    t0 *= t0; //                283: 16699404046508236100957035633283443518597649503553637437964659038273024
    t0 *= t0; //                284: 33398808093016472201914071266566887037195299007107274875929318076546048
    t0 *= t0; //                285: 66797616186032944403828142533133774074390598014214549751858636153092096
    t0 *= t0; //                286: 133595232372065888807656285066267548148781196028429099503717272306184192
    t0 *= t0; //                287: 267190464744131777615312570132535096297562392056858199007434544612368384
    t0 *= t0; //                288: 534380929488263555230625140265070192595124784113716398014869089224736768
    t0 *= t0; //                289: 1068761858976527110461250280530140385190249568227432796029738178449473536
    t0 *= t3; //                290: 1068761858976527110461250280530140385190249568227432796029738178449473935
    t0 *= t0; //                291: 2137523717953054220922500561060280770380499136454865592059476356898947870
    t0 *= t0; //                292: 4275047435906108441845001122120561540760998272909731184118952713797895740
    t0 *= t0; //                293: 8550094871812216883690002244241123081521996545819462368237905427595791480
    t0 *= t0; //                294: 17100189743624433767380004488482246163043993091638924736475810855191582960
    t0 *= t0; //                295: 34200379487248867534760008976964492326087986183277849472951621710383165920
    t0 *= t0; //                296: 68400758974497735069520017953928984652175972366555698945903243420766331840
    t0 *= t0; //                297: 136801517948995470139040035907857969304351944733111397891806486841532663680
    t0 *= t0; //                298: 273603035897990940278080071815715938608703889466222795783612973683065327360
    t0 *= t2; //                299: 273603035897990940278080071815715938608703889466222795783612973683065327607
    t0 *= t0; //                300: 547206071795981880556160143631431877217407778932445591567225947366130655214
    t0 *= t0; //                301: 1094412143591963761112320287262863754434815557864891183134451894732261310428
    t0 *= t0; //                302: 2188824287183927522224640574525727508869631115729782366268903789464522620856
    t0 *= t0; //                303: 4377648574367855044449281149051455017739262231459564732537807578929045241712
    t0 *= t0; //                304: 8755297148735710088898562298102910035478524462919129465075615157858090483424
    t0 * t1 //                  305: 8755297148735710088898562298102910035478524462919129465075615157858090483433
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::One;

    #[test]
    fn test_alpha() {
        let mut a = -Felt::one();
        for _ in 0..100 {
            assert_eq!(exp_inv_alpha(&a), a.pow(INV_ALPHA));
            a += a;
        }
    }
}