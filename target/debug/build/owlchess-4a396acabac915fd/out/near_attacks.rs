const KING_ATTACKS: [Bitboard; 64] = [
    /* 0*/ bb(0x0000000000000302),
    /* 1*/ bb(0x0000000000000705),
    /* 2*/ bb(0x0000000000000e0a),
    /* 3*/ bb(0x0000000000001c14),
    /* 4*/ bb(0x0000000000003828),
    /* 5*/ bb(0x0000000000007050),
    /* 6*/ bb(0x000000000000e0a0),
    /* 7*/ bb(0x000000000000c040),
    /* 8*/ bb(0x0000000000030203),
    /* 9*/ bb(0x0000000000070507),
    /*10*/ bb(0x00000000000e0a0e),
    /*11*/ bb(0x00000000001c141c),
    /*12*/ bb(0x0000000000382838),
    /*13*/ bb(0x0000000000705070),
    /*14*/ bb(0x0000000000e0a0e0),
    /*15*/ bb(0x0000000000c040c0),
    /*16*/ bb(0x0000000003020300),
    /*17*/ bb(0x0000000007050700),
    /*18*/ bb(0x000000000e0a0e00),
    /*19*/ bb(0x000000001c141c00),
    /*20*/ bb(0x0000000038283800),
    /*21*/ bb(0x0000000070507000),
    /*22*/ bb(0x00000000e0a0e000),
    /*23*/ bb(0x00000000c040c000),
    /*24*/ bb(0x0000000302030000),
    /*25*/ bb(0x0000000705070000),
    /*26*/ bb(0x0000000e0a0e0000),
    /*27*/ bb(0x0000001c141c0000),
    /*28*/ bb(0x0000003828380000),
    /*29*/ bb(0x0000007050700000),
    /*30*/ bb(0x000000e0a0e00000),
    /*31*/ bb(0x000000c040c00000),
    /*32*/ bb(0x0000030203000000),
    /*33*/ bb(0x0000070507000000),
    /*34*/ bb(0x00000e0a0e000000),
    /*35*/ bb(0x00001c141c000000),
    /*36*/ bb(0x0000382838000000),
    /*37*/ bb(0x0000705070000000),
    /*38*/ bb(0x0000e0a0e0000000),
    /*39*/ bb(0x0000c040c0000000),
    /*40*/ bb(0x0003020300000000),
    /*41*/ bb(0x0007050700000000),
    /*42*/ bb(0x000e0a0e00000000),
    /*43*/ bb(0x001c141c00000000),
    /*44*/ bb(0x0038283800000000),
    /*45*/ bb(0x0070507000000000),
    /*46*/ bb(0x00e0a0e000000000),
    /*47*/ bb(0x00c040c000000000),
    /*48*/ bb(0x0302030000000000),
    /*49*/ bb(0x0705070000000000),
    /*50*/ bb(0x0e0a0e0000000000),
    /*51*/ bb(0x1c141c0000000000),
    /*52*/ bb(0x3828380000000000),
    /*53*/ bb(0x7050700000000000),
    /*54*/ bb(0xe0a0e00000000000),
    /*55*/ bb(0xc040c00000000000),
    /*56*/ bb(0x0203000000000000),
    /*57*/ bb(0x0507000000000000),
    /*58*/ bb(0x0a0e000000000000),
    /*59*/ bb(0x141c000000000000),
    /*60*/ bb(0x2838000000000000),
    /*61*/ bb(0x5070000000000000),
    /*62*/ bb(0xa0e0000000000000),
    /*63*/ bb(0x40c0000000000000),
];

const KNIGHT_ATTACKS: [Bitboard; 64] = [
    /* 0*/ bb(0x0000000000020400),
    /* 1*/ bb(0x0000000000050800),
    /* 2*/ bb(0x00000000000a1100),
    /* 3*/ bb(0x0000000000142200),
    /* 4*/ bb(0x0000000000284400),
    /* 5*/ bb(0x0000000000508800),
    /* 6*/ bb(0x0000000000a01000),
    /* 7*/ bb(0x0000000000402000),
    /* 8*/ bb(0x0000000002040004),
    /* 9*/ bb(0x0000000005080008),
    /*10*/ bb(0x000000000a110011),
    /*11*/ bb(0x0000000014220022),
    /*12*/ bb(0x0000000028440044),
    /*13*/ bb(0x0000000050880088),
    /*14*/ bb(0x00000000a0100010),
    /*15*/ bb(0x0000000040200020),
    /*16*/ bb(0x0000000204000402),
    /*17*/ bb(0x0000000508000805),
    /*18*/ bb(0x0000000a1100110a),
    /*19*/ bb(0x0000001422002214),
    /*20*/ bb(0x0000002844004428),
    /*21*/ bb(0x0000005088008850),
    /*22*/ bb(0x000000a0100010a0),
    /*23*/ bb(0x0000004020002040),
    /*24*/ bb(0x0000020400040200),
    /*25*/ bb(0x0000050800080500),
    /*26*/ bb(0x00000a1100110a00),
    /*27*/ bb(0x0000142200221400),
    /*28*/ bb(0x0000284400442800),
    /*29*/ bb(0x0000508800885000),
    /*30*/ bb(0x0000a0100010a000),
    /*31*/ bb(0x0000402000204000),
    /*32*/ bb(0x0002040004020000),
    /*33*/ bb(0x0005080008050000),
    /*34*/ bb(0x000a1100110a0000),
    /*35*/ bb(0x0014220022140000),
    /*36*/ bb(0x0028440044280000),
    /*37*/ bb(0x0050880088500000),
    /*38*/ bb(0x00a0100010a00000),
    /*39*/ bb(0x0040200020400000),
    /*40*/ bb(0x0204000402000000),
    /*41*/ bb(0x0508000805000000),
    /*42*/ bb(0x0a1100110a000000),
    /*43*/ bb(0x1422002214000000),
    /*44*/ bb(0x2844004428000000),
    /*45*/ bb(0x5088008850000000),
    /*46*/ bb(0xa0100010a0000000),
    /*47*/ bb(0x4020002040000000),
    /*48*/ bb(0x0400040200000000),
    /*49*/ bb(0x0800080500000000),
    /*50*/ bb(0x1100110a00000000),
    /*51*/ bb(0x2200221400000000),
    /*52*/ bb(0x4400442800000000),
    /*53*/ bb(0x8800885000000000),
    /*54*/ bb(0x100010a000000000),
    /*55*/ bb(0x2000204000000000),
    /*56*/ bb(0x0004020000000000),
    /*57*/ bb(0x0008050000000000),
    /*58*/ bb(0x00110a0000000000),
    /*59*/ bb(0x0022140000000000),
    /*60*/ bb(0x0044280000000000),
    /*61*/ bb(0x0088500000000000),
    /*62*/ bb(0x0010a00000000000),
    /*63*/ bb(0x0020400000000000),
];

const WHITE_PAWN_ATTACKS: [Bitboard; 64] = [
    /* 0*/ bb(0x0000000000000000),
    /* 1*/ bb(0x0000000000000000),
    /* 2*/ bb(0x0000000000000000),
    /* 3*/ bb(0x0000000000000000),
    /* 4*/ bb(0x0000000000000000),
    /* 5*/ bb(0x0000000000000000),
    /* 6*/ bb(0x0000000000000000),
    /* 7*/ bb(0x0000000000000000),
    /* 8*/ bb(0x0000000000000002),
    /* 9*/ bb(0x0000000000000005),
    /*10*/ bb(0x000000000000000a),
    /*11*/ bb(0x0000000000000014),
    /*12*/ bb(0x0000000000000028),
    /*13*/ bb(0x0000000000000050),
    /*14*/ bb(0x00000000000000a0),
    /*15*/ bb(0x0000000000000040),
    /*16*/ bb(0x0000000000000200),
    /*17*/ bb(0x0000000000000500),
    /*18*/ bb(0x0000000000000a00),
    /*19*/ bb(0x0000000000001400),
    /*20*/ bb(0x0000000000002800),
    /*21*/ bb(0x0000000000005000),
    /*22*/ bb(0x000000000000a000),
    /*23*/ bb(0x0000000000004000),
    /*24*/ bb(0x0000000000020000),
    /*25*/ bb(0x0000000000050000),
    /*26*/ bb(0x00000000000a0000),
    /*27*/ bb(0x0000000000140000),
    /*28*/ bb(0x0000000000280000),
    /*29*/ bb(0x0000000000500000),
    /*30*/ bb(0x0000000000a00000),
    /*31*/ bb(0x0000000000400000),
    /*32*/ bb(0x0000000002000000),
    /*33*/ bb(0x0000000005000000),
    /*34*/ bb(0x000000000a000000),
    /*35*/ bb(0x0000000014000000),
    /*36*/ bb(0x0000000028000000),
    /*37*/ bb(0x0000000050000000),
    /*38*/ bb(0x00000000a0000000),
    /*39*/ bb(0x0000000040000000),
    /*40*/ bb(0x0000000200000000),
    /*41*/ bb(0x0000000500000000),
    /*42*/ bb(0x0000000a00000000),
    /*43*/ bb(0x0000001400000000),
    /*44*/ bb(0x0000002800000000),
    /*45*/ bb(0x0000005000000000),
    /*46*/ bb(0x000000a000000000),
    /*47*/ bb(0x0000004000000000),
    /*48*/ bb(0x0000020000000000),
    /*49*/ bb(0x0000050000000000),
    /*50*/ bb(0x00000a0000000000),
    /*51*/ bb(0x0000140000000000),
    /*52*/ bb(0x0000280000000000),
    /*53*/ bb(0x0000500000000000),
    /*54*/ bb(0x0000a00000000000),
    /*55*/ bb(0x0000400000000000),
    /*56*/ bb(0x0002000000000000),
    /*57*/ bb(0x0005000000000000),
    /*58*/ bb(0x000a000000000000),
    /*59*/ bb(0x0014000000000000),
    /*60*/ bb(0x0028000000000000),
    /*61*/ bb(0x0050000000000000),
    /*62*/ bb(0x00a0000000000000),
    /*63*/ bb(0x0040000000000000),
];

const BLACK_PAWN_ATTACKS: [Bitboard; 64] = [
    /* 0*/ bb(0x0000000000000200),
    /* 1*/ bb(0x0000000000000500),
    /* 2*/ bb(0x0000000000000a00),
    /* 3*/ bb(0x0000000000001400),
    /* 4*/ bb(0x0000000000002800),
    /* 5*/ bb(0x0000000000005000),
    /* 6*/ bb(0x000000000000a000),
    /* 7*/ bb(0x0000000000004000),
    /* 8*/ bb(0x0000000000020000),
    /* 9*/ bb(0x0000000000050000),
    /*10*/ bb(0x00000000000a0000),
    /*11*/ bb(0x0000000000140000),
    /*12*/ bb(0x0000000000280000),
    /*13*/ bb(0x0000000000500000),
    /*14*/ bb(0x0000000000a00000),
    /*15*/ bb(0x0000000000400000),
    /*16*/ bb(0x0000000002000000),
    /*17*/ bb(0x0000000005000000),
    /*18*/ bb(0x000000000a000000),
    /*19*/ bb(0x0000000014000000),
    /*20*/ bb(0x0000000028000000),
    /*21*/ bb(0x0000000050000000),
    /*22*/ bb(0x00000000a0000000),
    /*23*/ bb(0x0000000040000000),
    /*24*/ bb(0x0000000200000000),
    /*25*/ bb(0x0000000500000000),
    /*26*/ bb(0x0000000a00000000),
    /*27*/ bb(0x0000001400000000),
    /*28*/ bb(0x0000002800000000),
    /*29*/ bb(0x0000005000000000),
    /*30*/ bb(0x000000a000000000),
    /*31*/ bb(0x0000004000000000),
    /*32*/ bb(0x0000020000000000),
    /*33*/ bb(0x0000050000000000),
    /*34*/ bb(0x00000a0000000000),
    /*35*/ bb(0x0000140000000000),
    /*36*/ bb(0x0000280000000000),
    /*37*/ bb(0x0000500000000000),
    /*38*/ bb(0x0000a00000000000),
    /*39*/ bb(0x0000400000000000),
    /*40*/ bb(0x0002000000000000),
    /*41*/ bb(0x0005000000000000),
    /*42*/ bb(0x000a000000000000),
    /*43*/ bb(0x0014000000000000),
    /*44*/ bb(0x0028000000000000),
    /*45*/ bb(0x0050000000000000),
    /*46*/ bb(0x00a0000000000000),
    /*47*/ bb(0x0040000000000000),
    /*48*/ bb(0x0200000000000000),
    /*49*/ bb(0x0500000000000000),
    /*50*/ bb(0x0a00000000000000),
    /*51*/ bb(0x1400000000000000),
    /*52*/ bb(0x2800000000000000),
    /*53*/ bb(0x5000000000000000),
    /*54*/ bb(0xa000000000000000),
    /*55*/ bb(0x4000000000000000),
    /*56*/ bb(0x0000000000000000),
    /*57*/ bb(0x0000000000000000),
    /*58*/ bb(0x0000000000000000),
    /*59*/ bb(0x0000000000000000),
    /*60*/ bb(0x0000000000000000),
    /*61*/ bb(0x0000000000000000),
    /*62*/ bb(0x0000000000000000),
    /*63*/ bb(0x0000000000000000),
];
