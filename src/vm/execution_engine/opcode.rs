//Constants
pub(crate) const NOP: u8 = 0;
pub(crate) const ACONST_NULL: u8 = 1;
pub(crate) const ICONST_M1: u8 = 2;
pub(crate) const ICONST_0: u8 = 3;
pub(crate) const ICONST_1: u8 = 4;
pub(crate) const ICONST_2: u8 = 5;
pub(crate) const ICONST_3: u8 = 6;
pub(crate) const ICONST_4: u8 = 7;
pub(crate) const ICONST_5: u8 = 8;
pub(crate) const LCONST_0: u8 = 9;
pub(crate) const LCONST_1: u8 = 10;
pub(crate) const FCONST_0: u8 = 11;
pub(crate) const FCONST_1: u8 = 12;
pub(crate) const FCONST_2: u8 = 13;
pub(crate) const DCONST_0: u8 = 14;
pub(crate) const DCONST_1: u8 = 15;
pub(crate) const BIPUSH: u8 = 16;
pub(crate) const SIPUSH: u8 = 17;
pub(crate) const LDC: u8 = 18;
pub(crate) const LDC_W: u8 = 19;
pub(crate) const LDC2_W: u8 = 20;

// Loads
pub(crate) const ILOAD: u8 = 21;
pub(crate) const LLOAD: u8 = 22;
pub(crate) const FLOAD: u8 = 23;
pub(crate) const DLOAD: u8 = 24;
pub(crate) const ALOAD: u8 = 25;
pub(crate) const ILOAD_0: u8 = 26;
pub(crate) const ILOAD_1: u8 = 27;
pub(crate) const ILOAD_2: u8 = 28;
pub(crate) const ILOAD_3: u8 = 29;
pub(crate) const LLOAD_0: u8 = 30;
pub(crate) const LLOAD_1: u8 = 31;
pub(crate) const LLOAD_2: u8 = 32;
pub(crate) const LLOAD_3: u8 = 33;
pub(crate) const FLOAD_0: u8 = 34;
pub(crate) const FLOAD_1: u8 = 35;
pub(crate) const FLOAD_2: u8 = 36;
pub(crate) const FLOAD_3: u8 = 37;
pub(crate) const DLOAD_0: u8 = 38;
pub(crate) const DLOAD_1: u8 = 39;
pub(crate) const DLOAD_2: u8 = 40;
pub(crate) const DLOAD_3: u8 = 41;
pub(crate) const ALOAD_0: u8 = 42;
pub(crate) const ALOAD_1: u8 = 43;
pub(crate) const ALOAD_2: u8 = 44;
pub(crate) const ALOAD_3: u8 = 45;
pub(crate) const IALOAD: u8 = 46;
pub(crate) const LALOAD: u8 = 47;
pub(crate) const FALOAD: u8 = 48;
pub(crate) const DALOAD: u8 = 49;
pub(crate) const AALOAD: u8 = 50;
pub(crate) const BALOAD: u8 = 51;
pub(crate) const CALOAD: u8 = 52;
pub(crate) const SALOAD: u8 = 53;

// Stores
pub(crate) const ISTORE: u8 = 54;
pub(crate) const LSTORE: u8 = 55;
pub(crate) const FSTORE: u8 = 56;
pub(crate) const DSTORE: u8 = 57;
pub(crate) const ASTORE: u8 = 58;
pub(crate) const ISTORE_0: u8 = 59;
pub(crate) const ISTORE_1: u8 = 60;
pub(crate) const ISTORE_2: u8 = 61;
pub(crate) const ISTORE_3: u8 = 62;
pub(crate) const LSTORE_0: u8 = 63;
pub(crate) const LSTORE_1: u8 = 64;
pub(crate) const LSTORE_2: u8 = 65;
pub(crate) const LSTORE_3: u8 = 66;
pub(crate) const FSTORE_0: u8 = 67;
pub(crate) const FSTORE_1: u8 = 68;
pub(crate) const FSTORE_2: u8 = 69;
pub(crate) const FSTORE_3: u8 = 70;
pub(crate) const DSTORE_0: u8 = 71;
pub(crate) const DSTORE_1: u8 = 72;
pub(crate) const DSTORE_2: u8 = 73;
pub(crate) const DSTORE_3: u8 = 74;
pub(crate) const ASTORE_0: u8 = 75;
pub(crate) const ASTORE_1: u8 = 76;
pub(crate) const ASTORE_2: u8 = 77;
pub(crate) const ASTORE_3: u8 = 78;
pub(crate) const IASTORE: u8 = 79;
pub(crate) const LASTORE: u8 = 80;
pub(crate) const FASTORE: u8 = 81;
pub(crate) const DASTORE: u8 = 82;
pub(crate) const AASTORE: u8 = 83;
pub(crate) const BASTORE: u8 = 84;
pub(crate) const CASTORE: u8 = 85;
pub(crate) const SASTORE: u8 = 86;

// Stack
pub(crate) const POP: u8 = 87;
pub(crate) const POP2: u8 = 88;
pub(crate) const DUP: u8 = 89;
pub(crate) const DUP_X1: u8 = 90;
pub(crate) const DUP_X2: u8 = 91;
pub(crate) const DUP2: u8 = 92;
pub(crate) const DUP2_X1: u8 = 93;
pub(crate) const DUP2_X2: u8 = 94;
pub(crate) const SWAP: u8 = 95;

// Math
pub(crate) const IADD: u8 = 96;
pub(crate) const LADD: u8 = 97;
pub(crate) const FADD: u8 = 98;
pub(crate) const DADD: u8 = 99;
pub(crate) const ISUB: u8 = 100;
pub(crate) const LSUB: u8 = 101;
pub(crate) const FSUB: u8 = 102;
pub(crate) const DSUB: u8 = 103;
pub(crate) const IMUL: u8 = 104;
pub(crate) const LMUL: u8 = 105;
pub(crate) const FMUL: u8 = 106;
pub(crate) const DMUL: u8 = 107;
pub(crate) const IDIV: u8 = 108;
pub(crate) const LDIV: u8 = 109;
pub(crate) const FDIV: u8 = 110;
pub(crate) const DDIV: u8 = 111;
pub(crate) const IREM: u8 = 112;
pub(crate) const LREM: u8 = 113;
pub(crate) const FREM: u8 = 114;
pub(crate) const DREM: u8 = 115;
pub(crate) const INEG: u8 = 116;
pub(crate) const LNEG: u8 = 117;
pub(crate) const FNEG: u8 = 118;
pub(crate) const DNEG: u8 = 119;
pub(crate) const ISHL: u8 = 120;
pub(crate) const LSHL: u8 = 121;
pub(crate) const ISHR: u8 = 122;
pub(crate) const LSHR: u8 = 123;
pub(crate) const IUSHR: u8 = 124;
pub(crate) const LUSHR: u8 = 125;
pub(crate) const IAND: u8 = 126;
pub(crate) const LAND: u8 = 127;
pub(crate) const IOR: u8 = 128;
pub(crate) const LOR: u8 = 129;
pub(crate) const IXOR: u8 = 130;
pub(crate) const LXOR: u8 = 131;
pub(crate) const IINC: u8 = 132;

// Conversions
pub(crate) const I2L: u8 = 133;
pub(crate) const I2F: u8 = 134;
pub(crate) const I2D: u8 = 135;
pub(crate) const L2I: u8 = 136;
pub(crate) const L2F: u8 = 137;
pub(crate) const L2D: u8 = 138;
pub(crate) const F2I: u8 = 139;
pub(crate) const F2L: u8 = 140;
pub(crate) const F2D: u8 = 141;
pub(crate) const D2I: u8 = 142;
pub(crate) const D2L: u8 = 143;
pub(crate) const D2F: u8 = 144;
pub(crate) const I2B: u8 = 145;
pub(crate) const I2C: u8 = 146;
pub(crate) const I2S: u8 = 147;

// Comparisons
pub(crate) const LCMP: u8 = 148;
pub(crate) const FCMPL: u8 = 149;
pub(crate) const FCMPG: u8 = 150;
pub(crate) const DCMPL: u8 = 151;
pub(crate) const DCMPG: u8 = 152;
pub(crate) const IFEQ: u8 = 153;
pub(crate) const IFNE: u8 = 154;
pub(crate) const IFLT: u8 = 155;
pub(crate) const IFGE: u8 = 156;
pub(crate) const IFGT: u8 = 157;
pub(crate) const IFLE: u8 = 158;
pub(crate) const IF_ICMPEQ: u8 = 159;
pub(crate) const IF_ICMPNE: u8 = 160;
pub(crate) const IF_ICMPLT: u8 = 161;
pub(crate) const IF_ICMPGE: u8 = 162;
pub(crate) const IF_ICMPGT: u8 = 163;
pub(crate) const IF_ICMPLE: u8 = 164;
pub(crate) const IF_ACMPEQ: u8 = 165;
pub(crate) const IF_ACMPNE: u8 = 166;

// Control
pub(crate) const GOTO: u8 = 167;
#[allow(dead_code)]
pub(crate) const JSR: u8 = 168; // Obsolete since Java 6 SE
#[allow(dead_code)]
pub(crate) const RET: u8 = 169; // Obsolete since Java 6 SE
pub(crate) const TABLESWITCH: u8 = 170;
pub(crate) const LOOKUPSWITCH: u8 = 171;
pub(crate) const IRETURN: u8 = 172;
pub(crate) const LRETURN: u8 = 173;
pub(crate) const FRETURN: u8 = 174;
pub(crate) const DRETURN: u8 = 175;
pub(crate) const ARETURN: u8 = 176;
pub(crate) const RETURN: u8 = 177;

// References
pub(crate) const GETSTATIC: u8 = 178;
pub(crate) const PUTSTATIC: u8 = 179;
pub(crate) const GETFIELD: u8 = 180;
pub(crate) const PUTFIELD: u8 = 181;
pub(crate) const INVOKEVIRTUAL: u8 = 182;
pub(crate) const INVOKESPECIAL: u8 = 183;
pub(crate) const INVOKESTATIC: u8 = 184;
pub(crate) const INVOKEINTERFACE: u8 = 185;
pub(crate) const INVOKEDYNAMIC: u8 = 186;
pub(crate) const NEW: u8 = 187;
pub(crate) const NEWARRAY: u8 = 188;
pub(crate) const ANEWARRAY: u8 = 189;
pub(crate) const ARRAYLENGTH: u8 = 190;
pub(crate) const ATHROW: u8 = 191;
pub(crate) const CHECKCAST: u8 = 192;
pub(crate) const INSTANCEOF: u8 = 193;
pub(crate) const MONITORENTER: u8 = 194;
pub(crate) const MONITOREXIT: u8 = 195;

// Extended
pub(crate) const WIDE: u8 = 196;
pub(crate) const MULTIANEWARRAY: u8 = 197;
pub(crate) const IFNULL: u8 = 198;
pub(crate) const IFNONNULL: u8 = 199;
pub(crate) const GOTO_W: u8 = 200;
#[allow(dead_code)]
pub(crate) const JSR_W: u8 = 201; // Obsolete since Java 6 SE

// Reserved
#[allow(dead_code)]
pub(crate) const BREAKPOINT: u8 = 202;
#[allow(dead_code)]
pub(crate) const IMPDEP1: u8 = 254;
#[allow(dead_code)]
pub(crate) const IMPDEP2: u8 = 255;
