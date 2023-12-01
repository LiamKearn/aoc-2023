use aho_corasick::AhoCorasick;

fn main() {
    dbg!(part1(EXAMPLE_INPUT_1));
    dbg!(part1(ACTUAL_INPUT));

    dbg!(part2(EXAMPLE_INPUT_2));
    dbg!(part2(ACTUAL_INPUT));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut digit_iter = line.matches(char::is_numeric);

            {
                let first = digit_iter
                    .next()
                    .expect(format!("No digit found in line {}", i).as_str());
                let last = digit_iter.last().unwrap_or(first);
                // Eww:
                String::from(first) + last
            }
        })
        .map(|s| s.parse::<usize>().unwrap())
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let patterns = &[
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                "1", "2", "3", "4", "5", "6", "7", "8", "9",
            ];
            let ac = AhoCorasick::new(patterns).unwrap();
            let mut digit_iter = ac.find_iter(line).map(|s| {
                match &line[s.start()..s.end()] {
                    "one" => "1",
                    "two" => "2",
                    "three" => "3",
                    "four" => "4",
                    "five" => "5",
                    "six" => "6",
                    "seven" => "7",
                    "eight" => "8",
                    "nine" => "9",
                    "1" => "1",
                    "2" => "2",
                    "3" => "3",
                    "4" => "4",
                    "5" => "5",
                    "6" => "6",
                    "7" => "7",
                    "8" => "8",
                    "9" => "9",
                    _ => panic!("Unexpected pattern match"),
                }
            });

            {
                let first = digit_iter
                    .next()
                    .expect(format!("No digit found in line {}", i).as_str());
                let last = digit_iter.last().unwrap_or(first);
                // Eww:
                String::from(first) + last
            }
        })
        .map(|s| s.parse::<usize>().unwrap())
        .sum::<usize>()
}

const EXAMPLE_INPUT_1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

const EXAMPLE_INPUT_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

const ACTUAL_INPUT: &str = r#"sdpgz3five4seven6fiveh
876mbxbrntsfm
fivek5mfzrdxfbn66nine8eight
554qdg
ninevsgxnine6threesix8
4fivehmg614five
three6sdnttwothree3
two26four2
586dntdbtmfourrjnjdzptcfrpr3
dgkclmseven8
onepeight8sqzkrvvn
ninednnsjeight5c9hkzkhrgzcz4
42pchdjlnsxr688pvlgsr
vmzkvb5six4fiveg2
517fourone7kthdrcxm
sevenfourjgtwocsqzfbsqvb9flrlzxpx
3jqdxrqqzhkgg2fourthree9four
qjgfbnqjeight4onejm84sevensix
pgvthree1six15nineoneone
56ninefivefhnmmh5fdqcvttpsjseven2
sixtrvj12twomq
2jmxzzm9xhxmndgbhjlblhsnine75
pckdqrp8pgnmkgv9onetwoxlk
73mrflhnfzfxjqrqznj2
tfourninegnjhhdzsix6sevenfive
35zrgthreetwonesz
9fivebstlx42
6threenine
threebscrccrt94four3gvqdkdtjphmnmmf2
3seventwo3jmzxfmcnvjbnsgbgktwo
vhljqhzpbtxcssix3dbvttwo
4ninefour6sixpccqltcpvh
5fourthreejngfour
33rgkkfiveeight4xq
sixklhpptnnk8qvgbdcn
3bdtsjdfbmv8eightseven9sixqgfcmfpcone
vhckknzlvcpfvttnmnk5zcvs1eightnine
qheightwophxbnlv5nine8bknvnkdbps
7eight3gccqdgrltpkpt
qvjqtqffvtp2six
eight5ntzknggp5
eightone6fourkf
vlksmsxsljlnhtwo3mrc3five
one2eight
bqnjmjsznc67tdvtpdt
8five57sixfive
one4zhhlncnbncvzrsbsslnh
nqdftnsevenonellvpsdhrnrtrdjhbqscpd78
1fivefourffm3eightlcssevenclsjtb
7mqqgglzqmpk415gcggplprone1
2jprlqkpb1sixhfbvzjrgsrpjzbhtgrhpfour2
9twotwoqlvkrkhjthree44shvjxkpjgzgphgprflvn
eightgsonefivefive2
7sixmpnbgtmzrdbfive41
tkrcfive2eight23four
vgbqvj5mjplnfdsqpfone2hpxz
kvl7onehdvvfdghbsngrgn
tlsbhjblnpdlrtfour6btjcfmgdvtfive
6qnfivehnt31qcpmnhx
onesnqrgxstfmzhtln5bnldmvlqzsh
fourgn5fourggbddjsj34zncxtmxxvsvs
4lnr6xjkpzdgvsevengplkjdjtns
hzfplpdt9
sf2twolj8
6zghkzbfdjctpfp1
eight65
tvzmxzstsixmkvjkjl6eightninefour
fivephplggzkmfivetjfourmvcpnjxfvg58eightwoc
1fivehpleightfivethree4
zeightwo8
1jfcztzr5sixoneshmlpgtwofour
3zksglknjmvqfqhfiveeight76onemldlltn
9four4vtg32srrbqfczmnxdchtbvc
csdb92sevenvsnxdbd
4sevenseven6
8two8rm3fkbsvphhdrznine
trp149zxzrkk
948two9ninenine
cbmfivetwoninekqbkhkddc3rlfxlfh
btvcshj4seven
964sjxonesixgcjxmtnine
nb83fivegxnvxxrstqnine
fgoneightljvnfour5gfsix4sevensixt
6one97
three3two3sixtwoeight
kxonesix59xtmbqd5
dfqfcbv5mjeight
nine2nqhrsvkvzzkkvnn6oneightlbq
eight8rztdlm
ttcfdskrxlqlggmgfourthree11two4eightwopgz
8seventwooneightfcj
bmt8nine
rjzkjlgone7four2cffkgq6eightccsmvxhkk
6qkdznvrcdltdgpn75vvmvf
tcjhrgtlsixjsrbxdnseven3hkd2pnklcmshsf
mzptwone7onethree4tzthree
1sixgnvzbhscjgcr1gxfzpm
15twoqzplbsmfcnznrgrnthreetwo4
six815eightbdrone5five
three9pp21h3hzzg
fivetwothreezdhthree2dggsdgfdrx
mrqpblvqvfour5zkvvfjjbr8plzkh
cnxdvg8
pzxeightwogxxmtgs6
zcddqkhkjlfive4onexkdggcbfbqzxhfxqnb6
3jqncxxjjz7
bcntgllhg4
9sixpbgnine
eightsevensixfive45861
qhrqsppdxdnseventwo3fiveeightfour
fiveeight9
qvbvjzbfzrtcsxdrsxlr1fourjtqckdjg3
sixfour728onethree
fjponeight728fourndlcrm
8fpbcvbone8ltzprklvgdcjhhsvq3
8sixhqmsixxqmclmgvhfive5
sevenvvc376ckfrjjjcn
5onelxvs
6eightjtfphvprone1ztrxq2
6789bnfjqg
zfrpr12two
8eight6jxmvxhchj
81fiveeightfourpxmdcfbqhf
hpxnskbgbjm9hqllklsxq6fivecvsxvpl
xjbtp57two5onefmkbzdtwo
five6fourhpk1twosix
sixxkqhckjdffmsjqgt4xjtgq
five9rxrzgk1
z2seven
six6nine16three
four66lzqjdpndp5four
2zzsdpx6vfckpgvlprx
kmlfslspdkxp4smpqkcrdpfn
seven7xqtszl3oneninetwo3
ninethreegrxg78lpgstkpvckngrbd
746eighttwo8two3five
one7nine2fourqctmjggsgtlvj8
sqdvmfrssz9rglsczgbdk6djnvb
jrnkffhkdseven86xvfkvlnv1three2
3jgjdzz887
four6threefivetwohgfrcbtcpv1
8nine23twozhnbn2
3four9xmdqjnvscqvqftbbbqbgp8cpczg6
96twoh4lrczqsvtfrj3
hgjvlbzrdprtrfivedhhsevenfour8five
threethree2sevenvsmlpxbsspxrgkfour2
4threethree7ptffzx
1eight3nktcptcs
545
2hvgvkrtqtdfk9rvnklqqjdfcvvmvx6
qhzvtqlmghjjtpcjeightgbkqjpsjqjbhseven283
dlqnqbreight64668jjcsoneightxfn
84lqtmgjxgcr
118
hceightwobgcbsbtslf2onebhkdqlpvxxjpgsnmzfthree9
fxhxhdlkfrdhfslllsevenjvzdcpkclxx6four
8ppsvxfxpbv87threefour
lonenine4two2threenx
xtvtjbspvcrlrhbczkqt199
glspl5fivecfzjjqjktmvcnk2
pfgcqxjkkx7sixjcgmgszzbv1gnpvjmckfour
onebtvsctzzbxcvvvmltwotwo73
hpxl9dgn22twordrsfk
dsixbtsbccxgkheight6hfsix
foursevenqztbvqhbghmtlq876two
7sevenzdhsmmrdrmvzddp7sixeight14
one3fourtwoseven71
hbzs5threeeightjkjmkg91rsthree
3jfshzmtdrsonejtonelmjfkkjrmhzfxttm
gvcxznvtdtgkhrvbnlqsxbdq677
ffjqbssh65mpvkpqcgvmlzsevenlrbhfourone
77khln
21sevenhnine4fivevclbksgpxxhsz
fldn4oneeightknbvh43xjqnnkgxv
1fourtwo
dbvskdoneoneone44gs6
kqvmpr16twotfvjckstvg6
1msix522lbnt
2eightlgsfcttnchptgq1eight
onesjpcjmss15vqlzlkfqtpmmrkkbseventwo
1pqzgkhhfltwozqls
28twothreelmgqkpnfourfivex3
eightxvjtsevensevenpcrjtfcllzeight3mbjdjnklgxzzfdz
8brsfjhmbjl9hn
4eight68fivesix7
ninethree3
eightone4
nhhrrvvcr5oneninesix
8fourdzzbsxlsnlqfsjjjlf
one866onemgzhsix
stwo56tj4two8
9fourndxfjdptt
hgvxgninelbvvttxlqpljt77one
9sixsixbhtqsfgt
tworvghztldrhz98xgdgkbpfj
vfhtlmxkeightsxpfcvbvmmms5txnktxhj5
1njpfjngfone
nine4xcjdsmbdl
tdzjr493threeeight1
9eightgmcc
dsmr83svmgltgkkzpvtfmdgfjff2
qtm7four6fivejmbcnrfpv3pffqsrmcd
7zdmrnshs843sixfourfourfxxg
d898two66eightbrfrxmvx
11twophvzjqhslk3ppcbqxpxq
sevenqrfrngkndc8lrmtj3zhczpcrhxfjxrdp2
brbpcldn7sevenone2six9
jgsjnqjgmfcs7fourfive7dbtqsvj
2bb7hrtgbxsn5
xqvhhlzkninexhgvfvfrpgjq812
17eighthggxphbzz8zxlcxdrvvkxgb39
4sixglxd
dkeightwo1xmfjmvxxmglbsgkbphrll
558gprdd
kzjnpmthree3three
dkpjssgpsgglngk6zfrktlgchtqxbzcgxb
24four92fivegccfourxcnl
5xjc
vhmxtlhcrlrk59jgsix3
59twokqpjp
threefive65
sixdnqlxcdg9six
twoxgdmq7dhcxjnxr
threelxtbtjdbtonecpxtwo5sevenfive
fptqvt9onezqc
threec24onedxcjhninesevenbjxlhtgfmf
sixrmjbj87sevenn
8thltwosevenscjrsevensix9jhjvbb
nine1bbpflkrjhgbrpeighttwoseventxfjmhjvsj
snnddlskllxqcjtceight1gmm4tfcfccxmjg
ddjr3549
8twoseven7kpseven2dj9
hsmqtbrjgs5cbxjlsnzqcdldhlv4mf1
hkdxzfjvzrxnjnbfgrg4
cnjsnqmbzrnsbsevenfour3
eight25three4five1
jz5jxvmfrgttkkpxffbthreefive2nsix
4pdbdkx234
fiveseventhree2nine1eightf
pvgfour284mqtx2brsqgkgjlgvpkhxmd
bhtlhjrgv94twofour
dbbvkdhhqheight83xqfbhtxpqtsgpqmssevenone
one6beightfivebzgmgdq4six
bftwonekfgfsvdfxqlzrglm5cpmkpn
five8fourtfltzzkvxzpzqhb1gvhqkcp
2vfrzrhnfx6799
5mndmdcjsr63
72fgbbctshbrsix3tpggclvpf9bgsc
xdjqkrkfnh2threefvckhksevenfournx2
nineninekfcdjmfqsrgpb61f
5threeeightninesix1fournine
7dzkzn
two5twoxzngtwotwosixl
pjmqfour81eight4
rsjbbnqfrlppflstq3zxbhhn
rlsrv1seveneighteighttwo47
55sffiveseven8two39
675kjvqfdszmdvztvkczxbr
eightthreemrjtrfgqvsixtwo2kxnbzmnhnine
twoseventwo9rcvqmkxdvsevenone7
2threesvvtztllvl52rzcscj3
424eighttpjzxtx
8jgfivextbkqvdhl
tqrvp1gtpczqlpvr
fsmvrkdrnine9pk
jvfdlvnvsixthree8three22
one5jzmdfh9phqkvb
7twoneklt
5nineeightxjgtnkfxknine1twofk
31lkhbnzckbcmxvdpmqqjtbn755
2txsixfivezrxvkktnxlnsfive
bkjczmrcpqjxjfx4xkxfclthzthree
eightsevenjscz7flvltvkmqkthreevqgsg
tzlhm1lbmmnineqdpvxlv14threefour
1eightsmkmsqbs69x7chhmdd9
three18onexvh31sevenone
24sixshjtwo4
tjphmjzvfclddbfc4foursevenone9zrckvdqn
rbffkrsntqj2tf24gfrlzzsr4
7ninerktmfsfoursixd
four296six87five
zkrxfsvm5ninenkhc8
96mgvzrmxzpjfourlnsninenine
threepphgrmqlxone56
plt372
lgleightwojjvbkmhkvklscxrzpn6twovvsdzrsixxdcskxhn
p7six
sixthree1five6sixfoureight
2zjfndzqqzhkvqcfljpkdntdm4
sjldrrnv6npzqdnpktkshcqfh
four369xxvbqbrgvx42
dlqtsthreebmt91vfmjxk47jpdbkjr
gpdxdnine76nfrszxgnine
34seven
366tjcone6btwo
fivelvvstsjvrxppznsxkmjr7gkpmhrsfxxvmxtlf8
six1seven
5threernlthslpdjvjv1
1ninedbzjj
oneeightrdhsfrcd7dfour
9seven92hlssjltd2four
934qjgrkczclghnine5kck
hqk7pfqsbsevenseven
threefourckvkbtwomnbrcv6flqr64
qzmfhjdvjqjkrzdsvmdhpd61xphqddjgnsthree
96zspcjsixqzbcgmjrjbtk
6bgzmtlkhpsixfournrlrvghthreeeight
8qrvhrjlgjb3nine1ntcrx1
4onegk959
fcsczdfkfour9seven
ttkqsjgnrfive7vzbjr26pqlppfrxcndqlmrtpxqndgqp
bxnxvlkhd9
46sevenvnbsqvqpcthreeninesix
vdjltg8
five3onevtxxtseventwoeight7
4seven684hcvm1
two9klzeight8
q259
mfzsthree1
zmqfivegfqzsfxnp5tdp
zrpggrqldx221nineeightlbtjbtx
fiveblninegkgfive3two8
threetvlsjbmnfive83three
tsixbqffhzfive8
ctxbssevenbrcvxt9
ft5ninetvh8
fivegdqst1two
three16oneqs
ktfphzlgone7
884zjgkmtddggchgvfvg
fourgsnone1ninefvltlzcgcrjhrp63
threentwo92
xjlttwo6
nfhpnmff9foureight8
5nineqpdkbgdfourseven1three3
ltnmsixnhzfnckdzpnheightnine6three
sixthree9fourzflqn28
dknzbthreed3
58llxmhb
3ninesixftzmeightone8
3sconeone5
hmq6one4kzmkzjkvn9gvfbklp
fcmkpcjh1three6pxkmkvmm5
rffive7sixthree1xjqxeight2
33nine
3six55one
kpbnhpveight9tgx
b6pxfmcpnhgqlzgmncsix6
tprmltwovzvrseven9two4
ntnine1575seven
57onebk7four
5d394lgtjbzzdtdgttgk
4gkcone
sone177
fourtwoeight2gcbgjqtxgdcv
922seven75six
tjzzcdnmlqlsljpkfvprqqnrkeight6three6
7fourxzltksevenclbf7vjslggfszg
oneljktrrhpfxfpr6three55five
xrqmxvrn356v
threenbhhxsevenztcggns5
five4eight
65twosevenone
351twoxrmcv9
eightstptjbkzninesixnine79gkgmvpm
6vgltwoxcjfkvhl7psfsvkssevenrxlksjktltkmt
fiveninefivengspsnfz3
six5fivekonenineqnkgcpvhkr
ngndtzrzfnssvtdeightglnxvtjkqtcgn67
3fcxqmhvzqclxqvkxqxxknslfive
1twohxrmz23seventhreefour
jsgdzxlbcnpcppgdrhxjkrpnine5two2rh5
541onetwoone9
three672
858sevenbvdtqvrzrf
3bptkpprssevenbtbmsvzmntnhtnqdddfrchhhfour
5qxzkpnninegrdtrjvkrfive668
dbsmtnhknineseven8
eight6ninefourcrfqnlvfour
two1two7ninehqmz
9fivekhnine
rlfff265eight
5jtshhjcjkvfive
eight73934six
gnzpnineonedzdm3ninedlmdrng
rxvmnklbfthree1sixfour
eight1vc54qhdcgseven
ninenllv48hpdvqxpnfivenineseven
tjsmjlteightfivecxvfpxn3
xtddxtcxklrnpvtjsv5eight991twotwo
8dtlvdjclfive134
four4fouronecseven
5nine8onebsmnnphb
9fivemqone
394eight
zgpzcvslj652rkdptfm
hnktnnvxjljtmg55
s9tzgbhfknkvqpthbbshckc38fdkh
nineonegthreendtzvseven7chtclt
qctcphbsxmldvfourbbcphcszdmdghsssrspbjxgclpkjnine9vblnzjsvrj
tvthree18sixeight
8lghseven9
nhdlg32qbmbmsksql392twonelr
s4nineone
3eight6mn7j
six4bnntqxjbql4
fourdzdccrkmjjmknxnf7eightkjvrlzqtm4
3tfxlpzjxjfour7one7eight5c
pcbtb7zvtzdnzjn
eightbjmrjmdfdeight6dhdzgfvggn
five725
47659fblbqrbbzrthreefsn
nsxkcxp1fivethreehxq3
2eightvbpzttsqgronesix5
sixsix3zbtfsxhsjm8fivemmdhc6
three94one
pngbhg12
sixfsp1fiveseven
6five9two
577x56ttnrdm
mrqhz3cl5one7twosix
6vxcjd1dgvssttpbf
3cmxjhgxkbx4qxseightfivevclpvvt8gkcqq
sevenone49lzllzfxd
two3seven7vdclqnvfive3four
6sevenfoursevenone9
85eightpqeightwojmh
92three
twocfhbzdsixfive6ltnxjkz
84bfkfvxbp
4jkeight
83lcktgpntfourxgptpql
eightsevenznvjqj2ktckonecgslpdg
2onevkbhhltxppxhthfvrmkbxthree3
43grghdqconeoneseven
fivefsgzsvltdqpgxfdfxdxt6four4
eightcds3rpjdqvhqgjninethree
hb16sbg5ghglgqjcztdsdxfpk
foursix8five
9tworxpkbnnvzdzgtq9ssgsbfbrrqtwo
1jrtvfvbzd
five9threetwokdzmkfbkqggvrthpljm
3threeznps8fourvpfcxtjqpmnine
qxvqqntcpnfourseven1lfblgmmxsevenfpxjqbskgdonesixtwonezct
threefvqjfknqntzmtdkz8
56sixzmshnfdgd4
twotwo1bbsbsixhrhlrvz6
75c
three23four29
4five182btnjxqtvtd
54dhhg7nine
497hhtgmprmvkthdtwo
35onelgvtxqxrvq5nineone
1tggrpnsch432sixhc21
155cdjntgmmjms9psszbgfv
8jxqjcrhfchvnsixkcnqvjdsixsixjdsvc
one8six6
twovfghzcgsrznk1three2threeqlshlztdroneightm
1xppptflnineeighteightnn617
svpnine6jhqslblrzqonetkrxddjk
2rnxbqrrfiveoneightgsc
two5gcmkhxvklknthreethree
6onesixvnfxnfrh537one
sevenninetwo2eightthree
five2four2nine2xlz7kkbdtlp
pzdtwone9eight63fivercztdbnbfournine1
4sevenxnzvsjzj5three
37ninegkzrsbcbtwohrseight
5threeqklhpbzmmjgqnpkfldcsixgsfj9oneightml
4threevpsrffive2
rmbxxtwo5twofive1sixeight
72glqq2tjknnhlxlnpntrrjbc
34threehdrchqmzcdjcckg23
b4487
7one7
twosevennine55one19gcgxs
44npfpjn7
8htmkzqhht9
26zzkbq3ninemdhntftfprqvnvshzdqfnine
five7onefour2lhpgscf7ffmhgp
pxqp5onebmmnljxssnine48nqhz
six1nine
seven4cdknfkdhvbone6foureightseven1
twooneseventwoxzhlnfgthzffz9
bnvzpzlmp4five1
dddcjrmbeight8nine
six3rglfhkthree1jlddcddtfsix
1rxfourxfszvlq6eightsixqbhgrrhll1
eightfour25
15fournfourtbdnrjmf
ftrvckp47slrbnkkneightxqqs5
5ninesix
mgddc66onezjppzml6oneightvv
zknine7nine8three
kmkkfiveltrflvxdcxtworsfiveeight3
oneonemn7oneone
mxcchzfxseventhreehhqjthree1
jvnineone6fgzxqszthreetwo
ninegx9five2two1four6
2xzgxhh
811vqn
hxqmnfcvbctsvrfgs5xvzeightrbrfq5ksqcxxmrs
8xqxzq1six7fivezbskvznxx
16seven5
onenfdmcxtddscbmgvksseven6eight
xbnmkjgdpbqgnxhdjninesk4
xdmmqpn5lhvhvjmxtone
three2994plmtrfqstmfive
one3367nine
fourfour3khgmtcctvfj11fpcqvnine
48fourmxxfour
five8dtcqjjrcrg1three
drhjrf8eightthreebxgs
onesixdht2vrmgnzdbj1three
gpfglhctmxzm9fcxlgtsixseven
6rrrpl
seven1six5bfjpmqc
k87rsxpsjx1mjxsdchjlqseven4seven
8three1
sixfrbvgcvvxm5
zzf599
soneightthreeeight26fourhjxfxjg
94nine4nine2dmg
98sevennine
sixdkcpjglgkjnine9vphg2fivehlggxbnjx
sixthreevphh8
kdtwoneseveneight5211
txjsmlrsix72ljpkpnmgbnkckpvgtfx3tdkmslsix
45oneseven
4sixdgsixtwo7
ninethree3kmzs
5blqnxkkjxp
3khcjgxghgdqkjfthreeone
tnst57
8sixvvsxxmvmtwofourfjhtks4pdnccx1
3fiveoneonezghfkxpsnkggld1
9pkmpxrxlvz
sixmlqb2
6cssvpsffour15gsqvv8six
ngsz1
9eightninejkd882
rsftptwo41sixp6eight9
eight9988threecjeighttwo
tworf5nine969
6oneeightwod
threeqgcdsdvksix8four52
ktwone7fourtpskssrdsrsevenffpxshtksvzxffour
qlrmxzg2six
sldmxllptp36zeightvh3
onefive8zkvnnzkglzfpsfl
dzbnthsjtm8sixroneightcx
gcxgtsrpsv3ggcxqghc
1rsgjmfprrtwosvbscszlmgmq8
six9three
eighttpvjjstnkgone36
jrxvfivetsdmjmkz477
46four
mxblldfxdxbxgltjzvx283
fone97mszqxhseven5
9threenine4
sixvbshdqvjngeightthreerpfhbrx2
krbtwone4ninetwohx6mprpvkp1
cftlseven6eightnine
twoqdrnvvqgkvmlgljdljvbvc9
seveneightmbbdlsssrjhsffdlfll1
8pdkgszhfj
f8
2cvbxndjfournine194sixfive
qctzrvbfonethree6
two3foursevenbsixtwonvmpsdqfmjfb
qrcvxvdngmzrs7cjrpkhrhzsix4five3seven
83hone65
threefour4fourbbhgbzrdsix
tcnoneightms4nqp4351
4eightzscdfcfms5
9two9rqbkxggthreethreesevenhnnnq
mxngggxnineeightxjzmf3six9
99gscgrdz3fourthreeppkgczb
49258fivehkjlcxdq
vdzlp67bkhdfpjkh9
onethreetwo1foureighthltfive
3rpkvlrzd
threenineptdjgzmcthreefour7
2oneqrjhmqzk48
threehjqbmbgdn5
tbnjv46sixljhmsf7vtrnr7
eight98onepsmdcpkgssevenqrhc
8jjhzgm8four
fourseven4twobkbxrmjmzz7mnjzttsix6
ninedv4onefzc
seven5fivefive8scrmc5
3four656
42ggpjxtpfsflxffxdnscjbs
fivefivetwofour7seven39
37pvxpmszcjtgnvkh
eight71three2one
eightsixsevenone6
lrrqcm1cktlbhng2qjxfcchzfr
eightnineseveneight6pqvqdqmk
8cmxrcmjkhckvxbtlxzx2eight9cqzb51eightwos
zngbcxskdvnccgrhbcc688
844fkkvffrqsixgfdsccgtwosixgkmkp
nmmeightwo8ninencqlbbgcv6x
5352jvlnrcdj
4fourfour7gdkkseven6
bfxzk5fourqmgtmtjvmbcl
sevenqxdzcsnbneight88sixbnkqgsone
3twosix
xjdphbsfnqhqntcmsixtwokl4xcbmxgxnbp94
mjhhzggdltwoone3sixtwo1sixtk
5onevdrkgdsgr6bdzjlthree
ninesevenfhkgdlgvlcpsfourpm6sfive
bghpthree7pdgtvhzninefivezgsxfsngm
two878ffjrq39bfkbqlrsjj
three978
two1cxqlhsrr28
fivex6
86threevhrzcrfive2six
mghpkvxt7226
3262hxgseventwolqbtfttwo
6three5sevennbslxgvlxl1fivemdqtvp
ntwone5znpvscmkffour
tvtkjdrgv4
95ninefive7gzljcmxklthreebjkxjdkcdghnj
four6bgdsevenfiveeightfour
5rkkglzccsdqcp35seveneight2
hrdlgtslrtp2
3kvrlxntlthfour
61ccfqjdh4sevenkdcmone
lcsdjgkg4
sixxmfphssixninefivefive9fivefive
four52shspfjkppdng
8dp8fourhdcsmtfvxv
tqp8sevenkvvzpsxts6cqgmzzdxqvmeight2
36bpbjspbms8
nine8onehvbh
9vhthfzjcdv
ninezvdpv267
526vctfvbsfive
two5qcmzmgrhjm
cpdxfkqlhkthreerrbxzgbglmlvzqpk8
2threetncvhvpqtjsixeightninesbqzfrnhhf
2513
smrdxrzcssevenonerbsrkdbgxsevenjtbl1
cdhjhgmdlldvpc64fivebvljninesd
five41nine2four
sixsevenninekg91four4three
vhvqhxrlkstwonine5
threejrvchvlxd6sxtmnszjrsdxfpg6eight2two
7dmmrtzcfivet
fivenine4fourltrnsd2fourqpkxb
54four
9dhpnplgmmrfgpvdvh
skczmbgddnmz3mvssdeightonebfbrtzzkgnrpdv
threesixsh28seven94
three78nine9
one26fivejcjxtkrnine
ctkdmhtrjxeight7fourmzrnjxhdsthreeeight7
fttprqpsm3bxcggksfqssh
f2zvdvfnz14lgrmhq
nine58nine
dsfjxsixhmkfzbveight4xggfour
8mqldjrtttg2six688lfrg
five7kpzkf
qpjh31
lqnnrzvbrfd15
73four
three73seven7sixzfivexjkmnbf
96h2
spljjjz9four
516
dqkmkg478sixsevenmtwo
fourvmnmqhtbxptmmd6
eight96nttshld9vgrkdkcjjrt4
bvggdvfivemrbnntgv4three8nrbcb
4862
zpthreesixsixseveneight5czlkkbhd
fourxb5
vzdxoneknvvm7sixzd5fn
29onenine
rlzzdppslfivethreendjghpvpq813six
6eightvkxmr
399
4nbhjgrq41xjpjp
six2four42nine1mbtbrhcxq2
pqgrtqsfjnzbfq18fiverxltgd9eight
rgpkxtj6oneone3dmcbtxgtsrcfmxkd
md8drvkonespbkfsevennxgkh
two64one73nq
one7twossdmp
8mlfxjssseven
ghtninefive7fiveonefournlpsd
six38
kpfnnfour4five1679
hjtzgrvfmvone1xt21
4lgdztmfdrxqjsgflgnbldmhgvbcqcvbkd9sixthreenvgtmpkjnd
8onekjghzcvmlonetwo4
vzvgs5266jmrl5
bleight7
tkkhzconeszjvnkrbsveightlfqs18
srkzcmgseveneight43zhfbl8six4
qkmrmhxxb96rqrzfp2qcptqvjm99
tftwo6cqd
6fourxssnsfhcn
hgpfnbzxbxdjrbrjln5794lbgrs
sevenfour1gmvcfkzzfhpnffsfour4bcone
bfxbvcfj7
zsx32htdfsjzssix7
ssdnnd6kdmvqnffpvvpeight63five
4tznccdhjz7375x5xmgtnjb
282954threezhrhfcdcqbsgsv
87
twoonesixvczfhjmpdh2hkdh9
7rkhk
ktwonenine6tjczeight8kzkxmmhpbd5one
2gvfrrhneightsixnine4rxkvp
963reight
eightklkqcn3threetbcxhkm
qmlr5onegrcggpcsftmcphp
3ft49two
2xnsbrv9
lrcslkgkninehghhfive452zvlszp
fourtwoeight2one
6kjrcfivefqmtwoqrcpncpt2
rcdfk1eight1fournhzpvslq
sztlrrktt4four5onesbjvd
qdp3
vdmcnjkqjvn6fxnmzdrmsgbm
lhmonegqddt7sixrxcqpfbdfvbgrhjhf
sixone8
9bdgczsxq6four
keightwo1cmlshtwofourksgbvfxvb
lc3eightngglstsgqz6sevennine
7qmseven557drgtjcz
nqvhqtbqseven6fivefive2
six47gfjfkcgxone
qmxvfzhzckx3
nine4qdfreighteightfournine
86ninesix
xzjsdplszj3mrtlrtqonesix9
ksmfjng3nineggtfmnb
fourtwo5ninejtxdpthfv
foursixsevensixonemjhbjnffourq4
twodvfffeight9sevenninetwojj5
1one4nine
twofvcjszbmsltxh317vvq8six
onejfmqnlqnlzlh4
9sevengggskjzkr
7796twohqccmzbfneightkhtwoneh
zkrv48seventwo92five
6blv
2ztztlsqjvpm4
7lsvmnkhvjmknsndcrb5
lnttk8fivesevenvxqzrzfour
5twopzczlqrrgm8one
6xvhrpzkbkvsix
two21rvfnsdmqq
38jvthkzntnfr11three6one
7z
one5fivesix
gmmdgczbvbrqkfhtv73rktsshh
crmqmlbxs1five
klbqjnxlsix76
4rpvqnl3tbzrxmzt8sixthreefour
8pkmhdfourzzbfhxtpjggqvbtvrzfiver8
2one2tworbhjldv5
zvrbvpxqj96eight
onezvmczthblv4cgddmmhfivej
2zffhnpkbkhrmrksxjqfjztxrbts
qjrrkhplk2nl
ks73
mxtvxfqdphlvvkkkxntcffour34qcgns6
twoeightrpfpdksq8nine72gkxlvmqmlj
jkqeightwojzmfjdgt15ninehgthxf
one9grbzt
2pprfzbjp
kxnchhthreexthzfjqz53
f6four7mttnqsvzmbsqljdzqhcpnprscsggvvsevennine
5five1t
4seveneight3nine5
7sc
sevenonegmfcqmdv3tzvsgmeighteight
fqgmpklfjonevjppghglqcvzthree79six
bthreefivedqxsxlbfjjgzgbzdf21five
nineone3
five7tpfvdjvtpbtbrtsevenfoureightrfflhhgx
nine8one7dcgrczs1xrqxtfive4
rd5threefour1vpkcls
cshxmsz4two7onecgzkpfivelhcxgdpmx
sevensqrpfcfv93nt
onetqxjsrlzfhgt465two
fqdfvgfhdeight3four5sixseventhreetwoneqfd
krltgfglsevenzjdv4five6
5three54
84rsx
n58
1qc1three4twosix71
363
7bg5eightsxnkcdsdfx
2fivetwo
nineeight9seven
8nqqnine2
ljlmfzbzcbone48
nbpnjb6threesix69fourgjxxtzp7
7fourgrqj
ftvzmv4ck698
onenine6sixl
qvtrkhpnv62gslfivekzbzpnrqrm
9sixkjxsbzfnhj5gbdqrtczlhzjhzl
nine1seven
4sixqhnqvdseventhree1
8prqonespxk24jlxkh4
74mrhsmxnbnhbjrf
one6mm66jccznbtjnfour1
521fsvn
qqninethree756fgrckrnxk3
seven2twosixfourr
9ninechrpncvqfone1
87rfpsgp9thrsix
cjphxnxkhx6gqzngzzbd
vsl2seven2
3bgjzmqbvvbhvmmvcnnttsqqqcjngtxmdnrf8
gknfive14four3twoeight
xtvbhct7three622sl4eight
9fivefourbxxqpczdn4two
twonine3eightplqjnljhgthree
fivehlrxpvtxm5
1fjmzkdgmlvtxlcztblfknrqnsp5ddftwo
vvhxgmgtsixglcxsqlxnhhvldh7seven19
dqhjtgsbjtphkfsthree3one3bhbvvjxgp
sfxhzvttvjtvbsneight9eighttwo
26nine65one8
kbcbcssevenmq13sevensevenbhfjldzfvbckztfv
fsqmpnxrfn7tbxkfzfltctl7
two7twotwo3
3cqgzdxhggqlveightr1sevenqgxtptmfvrfour
qcmfn3
four8five2sixnrdt
onenine6kppc7seveneight27
3dv132
npjkktn2
eighteighthkkmtpcjzmnqxg5bxpfgrxvtqthree
fourfour19dfqpnzr
sixkl28twopgzgfvlvfnm
eightsixsixfhfljfkpg9
sevenfnxbmzr7ssfhvssnzsrfs
six3onenine
sixone25hdvmm
mnz2threecnsbvjsh376six
one4one
xbvxnvrppxsvppnd2rstpjnvmhglsdmbk1
tvlvk4shmlkqmmhfddbgbjfourcpx9qfhrrnine
4nrnxppcseven5
one86gchcvlfc5ninenjhpckffzfour
4q2jlccvmgcnj17
v5283seven2hpch2
sixeightjsjbclsxmhpxqcx78zgcnfqcnhtmfdc
six1jh
sjgtcrzq7ssgzxccj94sixqqsqv
twodone6five
3nine5seven
8zngrtcjeightfourzqzrbbhs7
pqtthc2dszssv
lcpgrkprhksvsg6
5seven14cckncpbqjr15
five76tslmpggzcseven49
5six4sjmbqhxxnine6five9
hslrsq3seven
threerhf8three5
4sixnineqtlstseventwoone4
mfdvqblctd7119xhjh6sevensix
kshxspjn378bhvqngjjgbsxbglf
sxpjbgrfivesixeight5jpvk
63fourm
nine372eightjhbjvmsix51
487sdkxfsevennine
3twofgnjpmsrtl
1pqxthreemvlbqlbmshzfiveone
2fxvnb62five
six96threethree1bfpbmgzt
fourthreerdjkhfmmzbqrkvlkt8eight9
5tzclzmfx5
seven5fourcxngd
dtbxxbhqfs4honeninesevenzdtvzcnmz
six94
tb8sfm7fngplpbprfour
52vlpsldp8ltspbsninefourcsrcp
two56
1rhxpdkgnhmlcpzrnkcpplpftbs
fivejjlfp11three7tdk2six
bkkltsfzhp4
xszczgrt33
6sevencjvbzrszx9twotwofive
eight5636eightxpd3
twoninefiveeightbmfhtgrksix5four
795bczlkm1vchhbgntpthreefmvfplk
zsl5rlffhdpz22
9twotscnjtttone3five6four
foursixthreeeight6rcrtzpr5
kc8eight
7fourqnmqeightlxnxlzvznsvsonefour
xnpcbth7eightfjxz4twofiveone
two3twotjmtbzc
knj995hhs18
flnine8qklfchbfivethreesp
1sevennine1onethreeseventhree
foureightdbtsntzzmsdone17eight
38sevenfkbm7nmmtrcxd5
1bmknc6eightfour8five
3eight2
nine5six11tqsh
6onehgfr5pbljsp
hrgbjqzmd667
2fhlqvk8fggmseventwoqthreeknt
1zpfmlgxbg49five9twondktfn
mbghltchlbghrplzdp1hhzfzvjonegszlbrqdfn
threechdshmnbfpfhvhlfourqfsix23
thcbzmmmbmcnineccvsrpqdgcbjldqdvhqqmzvz66
zmlxnine4zmbqkjxnclninefourjrddc
onegdnccqfn57dhsxqzpqhjnxzk
ncgvtt148nsgmd
2eightxjzvrgd2stsbfmdfp
2oneone84sevenonedl
eightseven51dmcrftxmgvjc3
mxstpzpstwo4njlffxpkfseven129sxn
foursixjdq2svlsscl9
4gvbkcrkpbqkfdxbtvfive
1fivefpgfptknhfivesevenhthrdkmqr
1vztq1fpqltwofoureightseven9
7threerkbjsmstwoxrnmcpgrzg
75lfjpfzhl
8jcdtqhc
twofngdlrpk7
bsskd18
oneninerq5one3hvsssmkfive
six1vbxsnktv
five63
jjkzhm6mnjqkzfpqeight6
6tvpfmvfivegvfbxc
oneqgfptrlqsr3qschjjxfour3
2155slkvtwobqsd2six
rrxtlsnthreesixfour38fourtdn
9onebptxrbfxtrldzxf
51one472five9
tt9914
3ncvnxdzzsllcxvrnsf4one4
1fourdnsjscnhzgsspdrone8five2
two3crndvmtgseightone
38tworrtprmtwo21zctwo
9sevenfivebcbdckflh58rbrftfv
jfknvsmpdtwoeightthzvqr3dqvmmxqps
eight9rdgmsdkz
752ninekjvqrm
fourthreekbkphksmhv2lrdjqrjtvk
9sixthreenine5csnndtgjtrbkdkqrccftxmzmj
3onefourfoursevenxdzcrhveight
hdnbvczvbmdqtk8four3879
sixsptcfmqsnzloneonefive5
tccqbtfjqglmvmnine53bgzzcqkfp
nfour3lmbvsls
foureightsix59nine7
3jpzvpzxqgrqdone
1one939
28vsdl
1fourxneightoneknsixmr
fivelmnpsfxmv47
lqgqvcczjmbrbgqpq47tjhgxptxfzdpkbc
sfmd895hsf2
976cjqtgqone69
3hdmdfrbm26sggqpfive1
259pzchvvrbcvhklhbr9five
ftnfivebzdkq8
6four3
threetxgc2htprtqqj5fouroneightlf
one492six7rzpxpfourone
twoeightnine8
sixeightxpcpfqmnlgseven7one
zzpkbmh6six46four739
ninefivejqnfqvmctwo1
three9sixqtdgqmfour24
fddrhjk2
bqrdqqtkv6sqqdthree
g4four8rhggvkmjxbhszh
hfmqlrvknklhvnxrqztffive6bndcljvgzlnc8
4ljbgjvllx6vvfc4
one3rrbseven3sevenpnnrnrz6
sbb2sixnine8
znlgdncm99twofmhmftsnlk
dppfive4fsnxctq
sevenrpxdcn1eightmvx
8ninefivegzk7ftqbceightwogfv
hxlxggmkrqzbtfmdqxfdnineeight71
ghtbmmrdfh4twoeight
fourfive14lgonegdxjrpx5four
61gdrpn7two1four
eightqhxvgqpvgxnmmonefourdpkqsmzm5k
fktwone4ninennnjdccftwothreetwo
seveneightmgqfcfczxsthrxhq3zcthsrxshddnlxronekdhqmmbhzd
nsbtggzgjx3eighttpkhkvknpsqxsevenvh8
1sevenseven7ld"#;
