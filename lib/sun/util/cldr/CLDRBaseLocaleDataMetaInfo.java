/*
 * Copyright (c) 2012, 2024, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.  Oracle designates this
 * particular file as subject to the "Classpath" exception as provided
 * by Oracle in the LICENSE file that accompanied this code.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy is included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */
package sun.util.cldr;

import java.util.HashMap;
import java.util.Locale;
import java.util.Map;
import sun.util.locale.provider.LocaleDataMetaInfo;
import sun.util.locale.provider.LocaleProviderAdapter;

public class CLDRBaseLocaleDataMetaInfo implements LocaleDataMetaInfo {
    private static final Map<Locale, String[]> parentLocalesMap = HashMap.newHashMap(9);
    private static final Map<String, String> languageAliasMap = HashMap.newHashMap(217);
    static final boolean nonlikelyScript = true; // package access from CLDRLocaleProviderAdapter

    static {
        parentLocalesMap.put(Locale.forLanguageTag("en-001"),
            new String[] {
                "en-150", "en-AG", "en-AI", "en-AU", "en-BB", "en-BM", "en-BS", "en-BW", 
                "en-BZ", "en-CC", "en-CK", "en-CM", "en-CX", "en-CY", "en-DG", "en-DM", 
                "en-ER", "en-FJ", "en-FK", "en-FM", "en-GB", "en-GD", "en-GG", "en-GH", 
                "en-GI", "en-GM", "en-GY", "en-HK", "en-ID", "en-IE", "en-IL", "en-IM", 
                "en-IN", "en-IO", "en-JE", "en-JM", "en-KE", "en-KI", "en-KN", "en-KY", 
                "en-LC", "en-LR", "en-LS", "en-MG", "en-MO", "en-MS", "en-MT", "en-MU", 
                "en-MV", "en-MW", "en-MY", "en-NA", "en-NF", "en-NG", "en-NR", "en-NU", 
                "en-NZ", "en-PG", "en-PK", "en-PN", "en-PW", "en-RW", "en-SB", "en-SC", 
                "en-SD", "en-SG", "en-SH", "en-SL", "en-SS", "en-SX", "en-SZ", "en-TC", 
                "en-TK", "en-TO", "en-TT", "en-TV", "en-TZ", "en-UG", "en-VC", "en-VG", 
                "en-VU", "en-WS", "en-ZA", "en-ZM", "en-ZW", 
            });
        parentLocalesMap.put(Locale.forLanguageTag("fr-HT"),
            new String[] {
                "ht", 
            });
        parentLocalesMap.put(Locale.forLanguageTag("zh-Hant-HK"),
            new String[] {
                "zh-Hant-MO", 
            });
        parentLocalesMap.put(Locale.forLanguageTag("en-IN"),
            new String[] {
                "hi-Latn", 
            });
        parentLocalesMap.put(Locale.forLanguageTag("en-150"),
            new String[] {
                "en-AT", "en-BE", "en-CH", "en-DE", "en-DK", "en-FI", "en-NL", "en-SE", 
                "en-SI", 
            });
        parentLocalesMap.put(Locale.forLanguageTag("es-419"),
            new String[] {
                "es-AR", "es-BO", "es-BR", "es-BZ", "es-CL", "es-CO", "es-CR", "es-CU", 
                "es-DO", "es-EC", "es-GT", "es-HN", "es-JP", "es-MX", "es-NI", "es-PA", 
                "es-PE", "es-PR", "es-PY", "es-SV", "es-US", "es-UY", "es-VE", 
            });
        parentLocalesMap.put(Locale.ROOT,
            new String[] {
                "az-Arab", "az-Cyrl", "bal-Latn", "blt-Latn", "bm-Nkoo", "bs-Cyrl", 
                "byn-Latn", "cu-Glag", "dje-Arab", "dyo-Arab", "en-Dsrt", "en-Shaw", 
                "ff-Adlm", "ff-Arab", "ha-Arab", "iu-Latn", "kk-Arab", "ks-Deva", 
                "ku-Arab", "kxv-Deva", "kxv-Orya", "kxv-Telu", "ky-Arab", "ky-Latn", 
                "ml-Arab", "mn-Mong", "mni-Mtei", "ms-Arab", "pa-Arab", "sat-Deva", 
                "sd-Deva", "sd-Khoj", "sd-Sind", "shi-Latn", "so-Arab", "sr-Latn", 
                "sw-Arab", "tg-Arab", "ug-Cyrl", "uz-Arab", "uz-Cyrl", "vai-Latn", 
                "wo-Arab", "yo-Arab", "yue-Hans", "zh-Hant", 
            });
        parentLocalesMap.put(Locale.forLanguageTag("pt-PT"),
            new String[] {
                "pt-AO", "pt-CH", "pt-CV", "pt-FR", "pt-GQ", "pt-GW", "pt-LU", "pt-MO", 
                "pt-MZ", "pt-ST", "pt-TL", 
            });
        parentLocalesMap.put(Locale.forLanguageTag("no"),
            new String[] {
                "nb", "nn", "no-NO", 
            });

        languageAliasMap.put("zh-cmn-Hant", "zh-Hant");
        languageAliasMap.put("mwj", "vaj");
        languageAliasMap.put("zh-cmn-Hans", "zh-Hans");
        languageAliasMap.put("tdu", "dtp");
        languageAliasMap.put("pmc", "huw");
        languageAliasMap.put("sgn-PT", "psr");
        languageAliasMap.put("leg", "enl");
        languageAliasMap.put("pmk", "crr");
        languageAliasMap.put("ccq", "rki");
        languageAliasMap.put("sca", "hle");
        languageAliasMap.put("scc", "sr");
        languageAliasMap.put("und-nynorsk", "und");
        languageAliasMap.put("sgn-CH-DE", "sgg");
        languageAliasMap.put("und-arevmda", "und");
        languageAliasMap.put("en-GB-oed", "en-GB-oxendict");
        languageAliasMap.put("pmu", "phr");
        languageAliasMap.put("baz", "nvo");
        languageAliasMap.put("jar", "jgk");
        languageAliasMap.put("yos", "zom");
        languageAliasMap.put("mgx", "jbk");
        languageAliasMap.put("scr", "hr");
        languageAliasMap.put("gli", "kzk");
        languageAliasMap.put("ktr", "dtp");
        languageAliasMap.put("dwl", "dbt");
        languageAliasMap.put("myd", "aog");
        languageAliasMap.put("i-klingon", "tlh");
        languageAliasMap.put("kdv", "zkd");
        languageAliasMap.put("i-pwn", "pwn");
        languageAliasMap.put("myt", "mry");
        languageAliasMap.put("sum", "ulw");
        languageAliasMap.put("sul", "sgd");
        languageAliasMap.put("zh-gan", "gan");
        languageAliasMap.put("aam", "aas");
        languageAliasMap.put("i-navajo", "nv");
        languageAliasMap.put("btb", "beb");
        languageAliasMap.put("tgg", "bjp");
        languageAliasMap.put("zh-yue", "yue");
        languageAliasMap.put("sh", "sr-Latn");
        languageAliasMap.put("ppa", "bfy");
        languageAliasMap.put("sgn-CO", "csn");
        languageAliasMap.put("i-default", "en-x-i-default");
        languageAliasMap.put("kvs", "gdj");
        languageAliasMap.put("und-xiang", "und");
        languageAliasMap.put("thc", "tpo");
        languageAliasMap.put("ibi", "opa");
        languageAliasMap.put("nln", "azd");
        languageAliasMap.put("sgn-BE-FR", "sfb");
        languageAliasMap.put("ppr", "lcq");
        languageAliasMap.put("asd", "snz");
        languageAliasMap.put("nlr", "nrk");
        languageAliasMap.put("i-enochian", "und-x-i-enochian");
        languageAliasMap.put("sgn-BR", "bzs");
        languageAliasMap.put("tl", "fil");
        languageAliasMap.put("ybd", "rki");
        languageAliasMap.put("sgn-SE", "swl");
        languageAliasMap.put("sgn-DK", "dsl");
        languageAliasMap.put("thw", "ola");
        languageAliasMap.put("kwq", "yam");
        languageAliasMap.put("thx", "oyb");
        languageAliasMap.put("kgc", "tdf");
        languageAliasMap.put("kgd", "ncq");
        languageAliasMap.put("jeg", "oyb");
        languageAliasMap.put("kgh", "kml");
        languageAliasMap.put("lii", "raq");
        languageAliasMap.put("dit", "dif");
        languageAliasMap.put("tid", "itd");
        languageAliasMap.put("sgn-DE", "gsg");
        languageAliasMap.put("kgm", "plu");
        languageAliasMap.put("tie", "ras");
        languageAliasMap.put("kxe", "tvd");
        languageAliasMap.put("sgl", "isk");
        languageAliasMap.put("kxl", "kru");
        languageAliasMap.put("pat", "kxr");
        languageAliasMap.put("djl", "dze");
        languageAliasMap.put("elp", "amq");
        languageAliasMap.put("hrr", "jal");
        languageAliasMap.put("sgn-US", "ase");
        languageAliasMap.put("adp", "dz");
        languageAliasMap.put("und-hakka", "und");
        languageAliasMap.put("prp", "gu");
        languageAliasMap.put("aue", "ktz");
        languageAliasMap.put("nns", "nbr");
        languageAliasMap.put("pry", "prt");
        languageAliasMap.put("xba", "cax");
        languageAliasMap.put("xrq", "dmw");
        languageAliasMap.put("nnx", "ngv");
        languageAliasMap.put("cel-gaulish", "xtg");
        languageAliasMap.put("dkl", "aqd");
        languageAliasMap.put("sgn-FR", "fsl");
        languageAliasMap.put("bgm", "bcg");
        languageAliasMap.put("zh-min", "nan-x-zh-min");
        languageAliasMap.put("xsj", "suj");
        languageAliasMap.put("tkk", "twm");
        languageAliasMap.put("yuu", "yug");
        languageAliasMap.put("kzh", "dgl");
        languageAliasMap.put("xss", "zko");
        languageAliasMap.put("kzj", "dtp");
        languageAliasMap.put("sgn-ES", "ssp");
        languageAliasMap.put("szd", "umi");
        languageAliasMap.put("gav", "dev");
        languageAliasMap.put("pcr", "adx");
        languageAliasMap.put("yen", "ynq");
        languageAliasMap.put("kzt", "dtp");
        languageAliasMap.put("bhk", "fbl");
        languageAliasMap.put("sgn-GR", "gss");
        languageAliasMap.put("gbc", "wny");
        languageAliasMap.put("cjr", "mom");
        languageAliasMap.put("llo", "ngt");
        languageAliasMap.put("mnt", "wnn");
        languageAliasMap.put("i-hak", "hak");
        languageAliasMap.put("sgn-GB", "bfi");
        languageAliasMap.put("unp", "wro");
        languageAliasMap.put("aa-saaho", "ssy");
        languageAliasMap.put("cka", "cmr");
        languageAliasMap.put("bic", "bir");
        languageAliasMap.put("tlw", "weo");
        languageAliasMap.put("und-hepburn-heploc", "und-alalc97");
        languageAliasMap.put("mof", "xnt");
        languageAliasMap.put("agp", "apf");
        languageAliasMap.put("lmm", "rmx");
        languageAliasMap.put("nad", "xny");
        languageAliasMap.put("uok", "ema");
        languageAliasMap.put("skk", "oyb");
        languageAliasMap.put("und-bokmal", "und");
        languageAliasMap.put("tmk", "tdg");
        languageAliasMap.put("puz", "pub");
        languageAliasMap.put("tmp", "tyj");
        languageAliasMap.put("bjd", "drl");
        languageAliasMap.put("gti", "nyc");
        languageAliasMap.put("sgn-ZA", "sfs");
        languageAliasMap.put("zir", "scv");
        languageAliasMap.put("sgn-IT", "ise");
        languageAliasMap.put("bjq", "bzc");
        languageAliasMap.put("tne", "kak");
        languageAliasMap.put("tnf", "fa-AF");
        languageAliasMap.put("nbf", "nru");
        languageAliasMap.put("sgn-IE", "isg");
        languageAliasMap.put("bkb", "ebk");
        languageAliasMap.put("und-arevela", "und");
        languageAliasMap.put("zh-min-nan", "nan");
        languageAliasMap.put("zh-guoyu", "zh");
        languageAliasMap.put("i-mingo", "see-x-i-mingo");
        languageAliasMap.put("cmk", "xch");
        languageAliasMap.put("ayx", "nun");
        languageAliasMap.put("sgn-JP", "jsl");
        languageAliasMap.put("hy-arevmda", "hyw");
        languageAliasMap.put("sgn-BE-NL", "vgt");
        languageAliasMap.put("nbx", "ekc");
        languageAliasMap.put("guv", "duz");
        languageAliasMap.put("oun", "vaj");
        languageAliasMap.put("ais", "ami");
        languageAliasMap.put("smd", "kmb");
        languageAliasMap.put("jw", "jv");
        languageAliasMap.put("zkb", "kjh");
        languageAliasMap.put("und-aaland", "und-AX");
        languageAliasMap.put("izi", "eza");
        languageAliasMap.put("ncp", "kdz");
        languageAliasMap.put("blg", "iba");
        languageAliasMap.put("snb", "iba");
        languageAliasMap.put("cnr", "sr-ME");
        languageAliasMap.put("ajp", "apc");
        languageAliasMap.put("yiy", "yrm");
        languageAliasMap.put("ajt", "aeb");
        languageAliasMap.put("nts", "pij");
        languageAliasMap.put("art-lojban", "jbo");
        languageAliasMap.put("und-lojban", "und");
        languageAliasMap.put("i-lux", "lb");
        languageAliasMap.put("gfx", "vaj");
        languageAliasMap.put("tpw", "tpn");
        languageAliasMap.put("zh-hakka", "hak");
        languageAliasMap.put("i-ami", "ami");
        languageAliasMap.put("zh-xiang", "hsn");
        languageAliasMap.put("koj", "kwv");
        languageAliasMap.put("daf", "dnj");
        languageAliasMap.put("coy", "pij");
        languageAliasMap.put("ggo", "esg");
        languageAliasMap.put("ggn", "gvr");
        languageAliasMap.put("mst", "mry");
        languageAliasMap.put("xia", "acn");
        languageAliasMap.put("dap", "njz");
        languageAliasMap.put("ggr", "gtu");
        languageAliasMap.put("i-tay", "tay");
        languageAliasMap.put("rmr", "emx");
        languageAliasMap.put("lak", "ksp");
        languageAliasMap.put("drh", "mn");
        languageAliasMap.put("mo", "ro");
        languageAliasMap.put("drr", "kzk");
        languageAliasMap.put("sgn-MX", "mfs");
        languageAliasMap.put("wgw", "wgb");
        languageAliasMap.put("drw", "fa-AF");
        languageAliasMap.put("ill", "ilm");
        languageAliasMap.put("kpp", "jkm");
        languageAliasMap.put("i-tsu", "tsu");
        languageAliasMap.put("ilw", "gal");
        languageAliasMap.put("cqu", "quh");
        languageAliasMap.put("tsf", "taj");
        languageAliasMap.put("zh-wuu", "wuu");
        languageAliasMap.put("sgn-NL", "dse");
        languageAliasMap.put("yma", "lrr");
        languageAliasMap.put("gio", "aou");
        languageAliasMap.put("meg", "cir");
        languageAliasMap.put("sgn-NO", "nsi");
        languageAliasMap.put("xkh", "waw");
        languageAliasMap.put("sgn-NI", "ncs");
        languageAliasMap.put("zh-cmn", "zh");
        languageAliasMap.put("ymt", "mtm");
        languageAliasMap.put("i-bnn", "bnn");
        languageAliasMap.put("i-tao", "tao");
        languageAliasMap.put("sap", "aqt");
        languageAliasMap.put("wit", "nol");
        languageAliasMap.put("wiw", "nwo");
        languageAliasMap.put("krm", "bmf");
        languageAliasMap.put("nxu", "bpp");
        languageAliasMap.put("dud", "uth");
        languageAliasMap.put("duj", "dwu");
        languageAliasMap.put("mwd", "dmw");
        languageAliasMap.put("und-saaho", "und");
    }

    private static class CLDRMapHolder {
        private static final Map<String, String> tzCanonicalIDMap = HashMap.newHashMap(451);
        private static final Map<String, String> likelyScriptMap = HashMap.newHashMap(117);

        static {
            tzCanonicalIDMap.put("fmksa", "Pacific/Kosrae");
            tzCanonicalIDMap.put("ruyks", "Asia/Yakutsk");
            tzCanonicalIDMap.put("ltvno", "Europe/Vilnius");
            tzCanonicalIDMap.put("caydq", "America/Dawson_Creek");
            tzCanonicalIDMap.put("brsao", "America/Sao_Paulo");
            tzCanonicalIDMap.put("Brazil/East", "America/Sao_Paulo");
            tzCanonicalIDMap.put("pwror", "Pacific/Palau");
            tzCanonicalIDMap.put("chzrh", "Europe/Zurich");
            tzCanonicalIDMap.put("svsal", "America/El_Salvador");
            tzCanonicalIDMap.put("vistt", "America/St_Thomas");
            tzCanonicalIDMap.put("America/Virgin", "America/St_Thomas");
            tzCanonicalIDMap.put("ruikt", "Asia/Irkutsk");
            tzCanonicalIDMap.put("gpmsb", "America/Marigot");
            tzCanonicalIDMap.put("aruaq", "America/Argentina/San_Juan");
            tzCanonicalIDMap.put("sesto", "Europe/Stockholm");
            tzCanonicalIDMap.put("uymvd", "America/Montevideo");
            tzCanonicalIDMap.put("mxmty", "America/Monterrey");
            tzCanonicalIDMap.put("esmad", "Europe/Madrid");
            tzCanonicalIDMap.put("grath", "Europe/Athens");
            tzCanonicalIDMap.put("aqcas", "Antarctica/Casey");
            tzCanonicalIDMap.put("bjptn", "Africa/Porto-Novo");
            tzCanonicalIDMap.put("cayda", "America/Dawson");
            tzCanonicalIDMap.put("pnpcn", "Pacific/Pitcairn");
            tzCanonicalIDMap.put("ruovb", "Asia/Novosibirsk");
            tzCanonicalIDMap.put("mxhmo", "America/Hermosillo");
            tzCanonicalIDMap.put("sjlyr", "Arctic/Longyearbyen");
            tzCanonicalIDMap.put("Atlantic/Jan_Mayen", "Arctic/Longyearbyen");
            tzCanonicalIDMap.put("uztas", "Asia/Tashkent");
            tzCanonicalIDMap.put("aqddu", "Antarctica/DumontDUrville");
            tzCanonicalIDMap.put("camon", "America/Moncton");
            tzCanonicalIDMap.put("gmt", "Etc/GMT");
            tzCanonicalIDMap.put("Etc/GMT+0", "Etc/GMT");
            tzCanonicalIDMap.put("Etc/GMT-0", "Etc/GMT");
            tzCanonicalIDMap.put("Etc/GMT0", "Etc/GMT");
            tzCanonicalIDMap.put("Etc/Greenwich", "Etc/GMT");
            tzCanonicalIDMap.put("GMT", "Etc/GMT");
            tzCanonicalIDMap.put("GMT+0", "Etc/GMT");
            tzCanonicalIDMap.put("GMT-0", "Etc/GMT");
            tzCanonicalIDMap.put("GMT0", "Etc/GMT");
            tzCanonicalIDMap.put("Greenwich", "Etc/GMT");
            tzCanonicalIDMap.put("usome", "America/Nome");
            tzCanonicalIDMap.put("uasip", "Europe/Simferopol");
            tzCanonicalIDMap.put("vnsgn", "Asia/Saigon");
            tzCanonicalIDMap.put("Asia/Ho_Chi_Minh", "Asia/Saigon");
            tzCanonicalIDMap.put("nimga", "America/Managua");
            tzCanonicalIDMap.put("joamm", "Asia/Amman");
            tzCanonicalIDMap.put("rwkgl", "Africa/Kigali");
            tzCanonicalIDMap.put("etadd", "Africa/Addis_Ababa");
            tzCanonicalIDMap.put("kgfru", "Asia/Bishkek");
            tzCanonicalIDMap.put("iddjj", "Asia/Jayapura");
            tzCanonicalIDMap.put("cayek", "America/Rankin_Inlet");
            tzCanonicalIDMap.put("sbhir", "Pacific/Guadalcanal");
            tzCanonicalIDMap.put("mzmpm", "Africa/Maputo");
            tzCanonicalIDMap.put("rugdx", "Asia/Magadan");
            tzCanonicalIDMap.put("brrbr", "America/Rio_Branco");
            tzCanonicalIDMap.put("America/Porto_Acre", "America/Rio_Branco");
            tzCanonicalIDMap.put("Brazil/Acre", "America/Rio_Branco");
            tzCanonicalIDMap.put("est5edt", "EST5EDT");
            tzCanonicalIDMap.put("cobog", "America/Bogota");
            tzCanonicalIDMap.put("aiaxa", "America/Anguilla");
            tzCanonicalIDMap.put("cayev", "America/Inuvik");
            tzCanonicalIDMap.put("hntgu", "America/Tegucigalpa");
            tzCanonicalIDMap.put("ptlis", "Europe/Lisbon");
            tzCanonicalIDMap.put("Portugal", "Europe/Lisbon");
            tzCanonicalIDMap.put("gugum", "Pacific/Guam");
            tzCanonicalIDMap.put("cynic", "Asia/Nicosia");
            tzCanonicalIDMap.put("Europe/Nicosia", "Asia/Nicosia");
            tzCanonicalIDMap.put("tglfw", "Africa/Lome");
            tzCanonicalIDMap.put("auhba", "Australia/Hobart");
            tzCanonicalIDMap.put("Australia/Tasmania", "Australia/Hobart");
            tzCanonicalIDMap.put("Australia/Currie", "Australia/Hobart");
            tzCanonicalIDMap.put("gldkshvn", "America/Danmarkshavn");
            tzCanonicalIDMap.put("sgsin", "Asia/Singapore");
            tzCanonicalIDMap.put("Singapore", "Asia/Singapore");
            tzCanonicalIDMap.put("usphx", "America/Phoenix");
            tzCanonicalIDMap.put("US/Arizona", "America/Phoenix");
            tzCanonicalIDMap.put("szqmn", "Africa/Mbabane");
            tzCanonicalIDMap.put("mkskp", "Europe/Skopje");
            tzCanonicalIDMap.put("pgraw", "Pacific/Bougainville");
            tzCanonicalIDMap.put("rubax", "Asia/Barnaul");
            tzCanonicalIDMap.put("pkkhi", "Asia/Karachi");
            tzCanonicalIDMap.put("ssjub", "Africa/Juba");
            tzCanonicalIDMap.put("glthu", "America/Thule");
            tzCanonicalIDMap.put("iodga", "Indian/Chagos");
            tzCanonicalIDMap.put("brfen", "America/Noronha");
            tzCanonicalIDMap.put("Brazil/DeNoronha", "America/Noronha");
            tzCanonicalIDMap.put("glgoh", "America/Godthab");
            tzCanonicalIDMap.put("America/Nuuk", "America/Godthab");
            tzCanonicalIDMap.put("jmkin", "America/Jamaica");
            tzCanonicalIDMap.put("Jamaica", "America/Jamaica");
            tzCanonicalIDMap.put("aumel", "Australia/Melbourne");
            tzCanonicalIDMap.put("Australia/Victoria", "Australia/Melbourne");
            tzCanonicalIDMap.put("auldh", "Australia/Lord_Howe");
            tzCanonicalIDMap.put("Australia/LHI", "Australia/Lord_Howe");
            tzCanonicalIDMap.put("bqkra", "America/Kralendijk");
            tzCanonicalIDMap.put("somgq", "Africa/Mogadishu");
            tzCanonicalIDMap.put("mrnkc", "Africa/Nouakchott");
            tzCanonicalIDMap.put("auldc", "Australia/Lindeman");
            tzCanonicalIDMap.put("azbak", "Asia/Baku");
            tzCanonicalIDMap.put("cnurc", "Asia/Urumqi");
            tzCanonicalIDMap.put("Asia/Kashgar", "Asia/Urumqi");
            tzCanonicalIDMap.put("shshn", "Atlantic/St_Helena");
            tzCanonicalIDMap.put("gtgua", "America/Guatemala");
            tzCanonicalIDMap.put("nglos", "Africa/Lagos");
            tzCanonicalIDMap.put("rukra", "Asia/Krasnoyarsk");
            tzCanonicalIDMap.put("pfgmr", "Pacific/Gambier");
            tzCanonicalIDMap.put("yeade", "Asia/Aden");
            tzCanonicalIDMap.put("caybx", "America/Blanc-Sablon");
            tzCanonicalIDMap.put("cahal", "America/Halifax");
            tzCanonicalIDMap.put("Canada/Atlantic", "America/Halifax");
            tzCanonicalIDMap.put("caycb", "America/Cambridge_Bay");
            tzCanonicalIDMap.put("cfbgf", "Africa/Bangui");
            tzCanonicalIDMap.put("thbkk", "Asia/Bangkok");
            tzCanonicalIDMap.put("lccas", "America/St_Lucia");
            tzCanonicalIDMap.put("usanc", "America/Anchorage");
            tzCanonicalIDMap.put("US/Alaska", "America/Anchorage");
            tzCanonicalIDMap.put("rsbeg", "Europe/Belgrade");
            tzCanonicalIDMap.put("aqdav", "Antarctica/Davis");
            tzCanonicalIDMap.put("uslax", "America/Los_Angeles");
            tzCanonicalIDMap.put("US/Pacific", "America/Los_Angeles");
            tzCanonicalIDMap.put("US/Pacific-New", "America/Los_Angeles");
            tzCanonicalIDMap.put("mxmzt", "America/Mazatlan");
            tzCanonicalIDMap.put("Mexico/BajaSur", "America/Mazatlan");
            tzCanonicalIDMap.put("tkfko", "Pacific/Fakaofo");
            tzCanonicalIDMap.put("unk", "Etc/Unknown");
            tzCanonicalIDMap.put("usboi", "America/Boise");
            tzCanonicalIDMap.put("tcgdt", "America/Grand_Turk");
            tzCanonicalIDMap.put("gygeo", "America/Guyana");
            tzCanonicalIDMap.put("dzalg", "Africa/Algiers");
            tzCanonicalIDMap.put("usmoc", "America/Kentucky/Monticello");
            tzCanonicalIDMap.put("altia", "Europe/Tirane");
            tzCanonicalIDMap.put("jesth", "Europe/Jersey");
            tzCanonicalIDMap.put("nlams", "Europe/Amsterdam");
            tzCanonicalIDMap.put("mmrgn", "Asia/Rangoon");
            tzCanonicalIDMap.put("Asia/Yangon", "Asia/Rangoon");
            tzCanonicalIDMap.put("kygec", "America/Cayman");
            tzCanonicalIDMap.put("djjib", "Africa/Djibouti");
            tzCanonicalIDMap.put("bbbgi", "America/Barbados");
            tzCanonicalIDMap.put("zmlun", "Africa/Lusaka");
            tzCanonicalIDMap.put("esceu", "Africa/Ceuta");
            tzCanonicalIDMap.put("mxchi", "America/Chihuahua");
            tzCanonicalIDMap.put("nzcht", "Pacific/Chatham");
            tzCanonicalIDMap.put("NZ-CHAT", "Pacific/Chatham");
            tzCanonicalIDMap.put("tvfun", "Pacific/Funafuti");
            tzCanonicalIDMap.put("slfna", "Africa/Freetown");
            tzCanonicalIDMap.put("usmnm", "America/Menominee");
            tzCanonicalIDMap.put("inccu", "Asia/Calcutta");
            tzCanonicalIDMap.put("Asia/Kolkata", "Asia/Calcutta");
            tzCanonicalIDMap.put("plwaw", "Europe/Warsaw");
            tzCanonicalIDMap.put("Poland", "Europe/Warsaw");
            tzCanonicalIDMap.put("aqsyw", "Antarctica/Syowa");
            tzCanonicalIDMap.put("arbue", "America/Buenos_Aires");
            tzCanonicalIDMap.put("America/Argentina/Buenos_Aires", "America/Buenos_Aires");
            tzCanonicalIDMap.put("mykul", "Asia/Kuala_Lumpur");
            tzCanonicalIDMap.put("rusred", "Asia/Srednekolymsk");
            tzCanonicalIDMap.put("mxtij", "America/Tijuana");
            tzCanonicalIDMap.put("America/Ensenada", "America/Tijuana");
            tzCanonicalIDMap.put("Mexico/BajaNorte", "America/Tijuana");
            tzCanonicalIDMap.put("America/Santa_Isabel", "America/Tijuana");
            tzCanonicalIDMap.put("rukvx", "Europe/Kirov");
            tzCanonicalIDMap.put("hrzag", "Europe/Zagreb");
            tzCanonicalIDMap.put("itrom", "Europe/Rome");
            tzCanonicalIDMap.put("papty", "America/Panama");
            tzCanonicalIDMap.put("gqssg", "Africa/Malabo");
            tzCanonicalIDMap.put("cuhav", "America/Havana");
            tzCanonicalIDMap.put("Cuba", "America/Havana");
            tzCanonicalIDMap.put("mnuln", "Asia/Ulaanbaatar");
            tzCanonicalIDMap.put("Asia/Ulan_Bator", "Asia/Ulaanbaatar");
            tzCanonicalIDMap.put("brpvh", "America/Porto_Velho");
            tzCanonicalIDMap.put("gblon", "Europe/London");
            tzCanonicalIDMap.put("Europe/Belfast", "Europe/London");
            tzCanonicalIDMap.put("GB", "Europe/London");
            tzCanonicalIDMap.put("GB-Eire", "Europe/London");
            tzCanonicalIDMap.put("gwoxb", "Africa/Bissau");
            tzCanonicalIDMap.put("kipho", "Pacific/Enderbury");
            tzCanonicalIDMap.put("Pacific/Kanton", "Pacific/Enderbury");
            tzCanonicalIDMap.put("ttpos", "America/Port_of_Spain");
            tzCanonicalIDMap.put("ommct", "Asia/Muscat");
            tzCanonicalIDMap.put("lrmlw", "Africa/Monrovia");
            tzCanonicalIDMap.put("imdgs", "Europe/Isle_of_Man");
            tzCanonicalIDMap.put("mxcjs", "America/Ciudad_Juarez");
            tzCanonicalIDMap.put("npktm", "Asia/Katmandu");
            tzCanonicalIDMap.put("Asia/Kathmandu", "Asia/Katmandu");
            tzCanonicalIDMap.put("robuh", "Europe/Bucharest");
            tzCanonicalIDMap.put("rukuf", "Europe/Samara");
            tzCanonicalIDMap.put("muplu", "Indian/Mauritius");
            tzCanonicalIDMap.put("aubne", "Australia/Brisbane");
            tzCanonicalIDMap.put("Australia/Queensland", "Australia/Brisbane");
            tzCanonicalIDMap.put("mqfdf", "America/Martinique");
            tzCanonicalIDMap.put("aueuc", "Australia/Eucla");
            tzCanonicalIDMap.put("mlbko", "Africa/Bamako");
            tzCanonicalIDMap.put("Africa/Timbuktu", "Africa/Bamako");
            tzCanonicalIDMap.put("audrw", "Australia/Darwin");
            tzCanonicalIDMap.put("Australia/North", "Australia/Darwin");
            tzCanonicalIDMap.put("asppg", "Pacific/Pago_Pago");
            tzCanonicalIDMap.put("Pacific/Samoa", "Pacific/Pago_Pago");
            tzCanonicalIDMap.put("US/Samoa", "Pacific/Pago_Pago");
            tzCanonicalIDMap.put("lkcmb", "Asia/Colombo");
            tzCanonicalIDMap.put("lvrix", "Europe/Riga");
            tzCanonicalIDMap.put("lbbey", "Asia/Beirut");
            tzCanonicalIDMap.put("sdkrt", "Africa/Khartoum");
            tzCanonicalIDMap.put("aganu", "America/Antigua");
            tzCanonicalIDMap.put("frpar", "Europe/Paris");
            tzCanonicalIDMap.put("cmdla", "Africa/Douala");
            tzCanonicalIDMap.put("kwkwi", "Asia/Kuwait");
            tzCanonicalIDMap.put("jptyo", "Asia/Tokyo");
            tzCanonicalIDMap.put("Japan", "Asia/Tokyo");
            tzCanonicalIDMap.put("kitrw", "Pacific/Tarawa");
            tzCanonicalIDMap.put("cayyn", "America/Swift_Current");
            tzCanonicalIDMap.put("sttms", "Africa/Sao_Tome");
            tzCanonicalIDMap.put("metgd", "Europe/Podgorica");
            tzCanonicalIDMap.put("bddac", "Asia/Dhaka");
            tzCanonicalIDMap.put("Asia/Dacca", "Asia/Dhaka");
            tzCanonicalIDMap.put("cator", "America/Toronto");
            tzCanonicalIDMap.put("America/Montreal", "America/Toronto");
            tzCanonicalIDMap.put("Canada/Eastern", "America/Toronto");
            tzCanonicalIDMap.put("America/Nipigon", "America/Toronto");
            tzCanonicalIDMap.put("America/Thunder_Bay", "America/Toronto");
            tzCanonicalIDMap.put("cgbzv", "Africa/Brazzaville");
            tzCanonicalIDMap.put("tldil", "Asia/Dili");
            tzCanonicalIDMap.put("jeruslm", "Asia/Jerusalem");
            tzCanonicalIDMap.put("Asia/Tel_Aviv", "Asia/Jerusalem");
            tzCanonicalIDMap.put("Israel", "Asia/Jerusalem");
            tzCanonicalIDMap.put("gsgrv", "Atlantic/South_Georgia");
            tzCanonicalIDMap.put("usyak", "America/Yakutat");
            tzCanonicalIDMap.put("brbvb", "America/Boa_Vista");
            tzCanonicalIDMap.put("utc", "Etc/UTC");
            tzCanonicalIDMap.put("Etc/UCT", "Etc/UTC");
            tzCanonicalIDMap.put("Etc/Universal", "Etc/UTC");
            tzCanonicalIDMap.put("Etc/Zulu", "Etc/UTC");
            tzCanonicalIDMap.put("UCT", "Etc/UTC");
            tzCanonicalIDMap.put("UTC", "Etc/UTC");
            tzCanonicalIDMap.put("Universal", "Etc/UTC");
            tzCanonicalIDMap.put("Zulu", "Etc/UTC");
            tzCanonicalIDMap.put("cdfih", "Africa/Kinshasa");
            tzCanonicalIDMap.put("veccs", "America/Caracas");
            tzCanonicalIDMap.put("bmbda", "Atlantic/Bermuda");
            tzCanonicalIDMap.put("dmdom", "America/Dominica");
            tzCanonicalIDMap.put("mtmla", "Europe/Malta");
            tzCanonicalIDMap.put("idjkt", "Asia/Jakarta");
            tzCanonicalIDMap.put("mwblz", "Africa/Blantyre");
            tzCanonicalIDMap.put("msmni", "America/Montserrat");
            tzCanonicalIDMap.put("pyasu", "America/Asuncion");
            tzCanonicalIDMap.put("khpnh", "Asia/Phnom_Penh");
            tzCanonicalIDMap.put("aubhq", "Australia/Broken_Hill");
            tzCanonicalIDMap.put("Australia/Yancowinna", "Australia/Broken_Hill");
            tzCanonicalIDMap.put("artuc", "America/Argentina/Tucuman");
            tzCanonicalIDMap.put("cayxy", "America/Whitehorse");
            tzCanonicalIDMap.put("Canada/Yukon", "America/Whitehorse");
            tzCanonicalIDMap.put("lytip", "Africa/Tripoli");
            tzCanonicalIDMap.put("Libya", "Africa/Tripoli");
            tzCanonicalIDMap.put("basjj", "Europe/Sarajevo");
            tzCanonicalIDMap.put("knbas", "America/St_Kitts");
            tzCanonicalIDMap.put("smsai", "Europe/San_Marino");
            tzCanonicalIDMap.put("cnsha", "Asia/Shanghai");
            tzCanonicalIDMap.put("Asia/Chongqing", "Asia/Shanghai");
            tzCanonicalIDMap.put("Asia/Chungking", "Asia/Shanghai");
            tzCanonicalIDMap.put("Asia/Harbin", "Asia/Shanghai");
            tzCanonicalIDMap.put("PRC", "Asia/Shanghai");
            tzCanonicalIDMap.put("arctc", "America/Catamarca");
            tzCanonicalIDMap.put("America/Argentina/Catamarca", "America/Catamarca");
            tzCanonicalIDMap.put("America/Argentina/ComodRivadavia", "America/Catamarca");
            tzCanonicalIDMap.put("lavte", "Asia/Vientiane");
            tzCanonicalIDMap.put("fmpni", "Pacific/Ponape");
            tzCanonicalIDMap.put("Pacific/Pohnpei", "Pacific/Ponape");
            tzCanonicalIDMap.put("tzdar", "Africa/Dar_es_Salaam");
            tzCanonicalIDMap.put("ruvog", "Europe/Volgograd");
            tzCanonicalIDMap.put("ghacc", "Africa/Accra");
            tzCanonicalIDMap.put("deber", "Europe/Berlin");
            tzCanonicalIDMap.put("cyfmg", "Asia/Famagusta");
            tzCanonicalIDMap.put("bfoua", "Africa/Ouagadougou");
            tzCanonicalIDMap.put("ruuly", "Europe/Ulyanovsk");
            tzCanonicalIDMap.put("prsju", "America/Puerto_Rico");
            tzCanonicalIDMap.put("auper", "Australia/Perth");
            tzCanonicalIDMap.put("Australia/West", "Australia/Perth");
            tzCanonicalIDMap.put("mnhvd", "Asia/Hovd");
            tzCanonicalIDMap.put("sydam", "Asia/Damascus");
            tzCanonicalIDMap.put("ecgye", "America/Guayaquil");
            tzCanonicalIDMap.put("eetll", "Europe/Tallinn");
            tzCanonicalIDMap.put("braux", "America/Araguaina");
            tzCanonicalIDMap.put("nrinu", "Pacific/Nauru");
            tzCanonicalIDMap.put("ugkla", "Africa/Kampala");
            tzCanonicalIDMap.put("gazastrp", "Asia/Gaza");
            tzCanonicalIDMap.put("zajnb", "Africa/Johannesburg");
            tzCanonicalIDMap.put("cayzs", "America/Coral_Harbour");
            tzCanonicalIDMap.put("America/Atikokan", "America/Coral_Harbour");
            tzCanonicalIDMap.put("mxmam", "America/Matamoros");
            tzCanonicalIDMap.put("totbu", "Pacific/Tongatapu");
            tzCanonicalIDMap.put("htpap", "America/Port-au-Prince");
            tzCanonicalIDMap.put("uslui", "America/Louisville");
            tzCanonicalIDMap.put("America/Kentucky/Louisville", "America/Louisville");
            tzCanonicalIDMap.put("dkcph", "Europe/Copenhagen");
            tzCanonicalIDMap.put("usden", "America/Denver");
            tzCanonicalIDMap.put("America/Shiprock", "America/Denver");
            tzCanonicalIDMap.put("Navajo", "America/Denver");
            tzCanonicalIDMap.put("US/Mountain", "America/Denver");
            tzCanonicalIDMap.put("cccck", "Indian/Cocos");
            tzCanonicalIDMap.put("eheai", "Africa/El_Aaiun");
            tzCanonicalIDMap.put("hkhkg", "Asia/Hong_Kong");
            tzCanonicalIDMap.put("Hongkong", "Asia/Hong_Kong");
            tzCanonicalIDMap.put("usdet", "America/Detroit");
            tzCanonicalIDMap.put("US/Michigan", "America/Detroit");
            tzCanonicalIDMap.put("ptpdl", "Atlantic/Azores");
            tzCanonicalIDMap.put("sndkr", "Africa/Dakar");
            tzCanonicalIDMap.put("ustel", "America/Indiana/Tell_City");
            tzCanonicalIDMap.put("pfnhv", "Pacific/Marquesas");
            tzCanonicalIDMap.put("pelim", "America/Lima");
            tzCanonicalIDMap.put("silju", "Europe/Ljubljana");
            tzCanonicalIDMap.put("usind", "America/Indianapolis");
            tzCanonicalIDMap.put("America/Fort_Wayne", "America/Indianapolis");
            tzCanonicalIDMap.put("America/Indiana/Indianapolis", "America/Indianapolis");
            tzCanonicalIDMap.put("US/East-Indiana", "America/Indianapolis");
            tzCanonicalIDMap.put("bzbze", "America/Belize");
            tzCanonicalIDMap.put("eslpa", "Atlantic/Canary");
            tzCanonicalIDMap.put("brssa", "America/Bahia");
            tzCanonicalIDMap.put("kzksn", "Asia/Qostanay");
            tzCanonicalIDMap.put("aedxb", "Asia/Dubai");
            tzCanonicalIDMap.put("aqrot", "Antarctica/Rothera");
            tzCanonicalIDMap.put("vavat", "Europe/Vatican");
            tzCanonicalIDMap.put("arush", "America/Argentina/Ushuaia");
            tzCanonicalIDMap.put("afkbl", "Asia/Kabul");
            tzCanonicalIDMap.put("arcor", "America/Cordoba");
            tzCanonicalIDMap.put("America/Argentina/Cordoba", "America/Cordoba");
            tzCanonicalIDMap.put("America/Rosario", "America/Cordoba");
            tzCanonicalIDMap.put("vcsvd", "America/St_Vincent");
            tzCanonicalIDMap.put("usnyc", "America/New_York");
            tzCanonicalIDMap.put("US/Eastern", "America/New_York");
            tzCanonicalIDMap.put("fkpsy", "Atlantic/Stanley");
            tzCanonicalIDMap.put("usndnsl", "America/North_Dakota/New_Salem");
            tzCanonicalIDMap.put("uswlz", "America/Indiana/Winamac");
            tzCanonicalIDMap.put("gigib", "Europe/Gibraltar");
            tzCanonicalIDMap.put("brstm", "America/Santarem");
            tzCanonicalIDMap.put("bymsq", "Europe/Minsk");
            tzCanonicalIDMap.put("aqplm", "Antarctica/Palmer");
            tzCanonicalIDMap.put("uaiev", "Europe/Kiev");
            tzCanonicalIDMap.put("Europe/Kyiv", "Europe/Kiev");
            tzCanonicalIDMap.put("Europe/Zaporozhye", "Europe/Kiev");
            tzCanonicalIDMap.put("Europe/Uzhgorod", "Europe/Kiev");
            tzCanonicalIDMap.put("kmyva", "Indian/Comoro");
            tzCanonicalIDMap.put("rutof", "Asia/Tomsk");
            tzCanonicalIDMap.put("egcai", "Africa/Cairo");
            tzCanonicalIDMap.put("Egypt", "Africa/Cairo");
            tzCanonicalIDMap.put("mxoji", "America/Ojinaga");
            tzCanonicalIDMap.put("dosdq", "America/Santo_Domingo");
            tzCanonicalIDMap.put("sxphi", "America/Lower_Princes");
            tzCanonicalIDMap.put("kicxi", "Pacific/Kiritimati");
            tzCanonicalIDMap.put("bolpb", "America/La_Paz");
            tzCanonicalIDMap.put("mvmle", "Indian/Maldives");
            tzCanonicalIDMap.put("mxmex", "America/Mexico_City");
            tzCanonicalIDMap.put("Mexico/General", "America/Mexico_City");
            tzCanonicalIDMap.put("ushnl", "Pacific/Honolulu");
            tzCanonicalIDMap.put("US/Hawaii", "Pacific/Honolulu");
            tzCanonicalIDMap.put("Pacific/Johnston", "Pacific/Honolulu");
            tzCanonicalIDMap.put("casjf", "America/St_Johns");
            tzCanonicalIDMap.put("Canada/Newfoundland", "America/St_Johns");
            tzCanonicalIDMap.put("nzakl", "Pacific/Auckland");
            tzCanonicalIDMap.put("Antarctica/South_Pole", "Pacific/Auckland");
            tzCanonicalIDMap.put("NZ", "Pacific/Auckland");
            tzCanonicalIDMap.put("mst7mdt", "MST7MDT");
            tzCanonicalIDMap.put("mncoq", "Asia/Choibalsan");
            tzCanonicalIDMap.put("ptfnc", "Atlantic/Madeira");
            tzCanonicalIDMap.put("auadl", "Australia/Adelaide");
            tzCanonicalIDMap.put("Australia/South", "Australia/Adelaide");
            tzCanonicalIDMap.put("armdz", "America/Mendoza");
            tzCanonicalIDMap.put("America/Argentina/Mendoza", "America/Mendoza");
            tzCanonicalIDMap.put("pmmqc", "America/Miquelon");
            tzCanonicalIDMap.put("ruchita", "Asia/Chita");
            tzCanonicalIDMap.put("ytmam", "Indian/Mayotte");
            tzCanonicalIDMap.put("mxmid", "America/Merida");
            tzCanonicalIDMap.put("caiql", "America/Iqaluit");
            tzCanonicalIDMap.put("America/Pangnirtung", "America/Iqaluit");
            tzCanonicalIDMap.put("mxcun", "America/Cancun");
            tzCanonicalIDMap.put("ummdy", "Pacific/Midway");
            tzCanonicalIDMap.put("arrgl", "America/Argentina/Rio_Gallegos");
            tzCanonicalIDMap.put("utcw10", "Etc/GMT+10");
            tzCanonicalIDMap.put("HST", "Etc/GMT+10");
            tzCanonicalIDMap.put("brern", "America/Eirunepe");
            tzCanonicalIDMap.put("utcw11", "Etc/GMT+11");
            tzCanonicalIDMap.put("utcw12", "Etc/GMT+12");
            tzCanonicalIDMap.put("wfmau", "Pacific/Wallis");
            tzCanonicalIDMap.put("lsmsu", "Africa/Maseru");
            tzCanonicalIDMap.put("brmao", "America/Manaus");
            tzCanonicalIDMap.put("Brazil/West", "America/Manaus");
            tzCanonicalIDMap.put("cdfbm", "Africa/Lubumbashi");
            tzCanonicalIDMap.put("nawdh", "Africa/Windhoek");
            tzCanonicalIDMap.put("bebru", "Europe/Brussels");
            tzCanonicalIDMap.put("kzaau", "Asia/Aqtau");
            tzCanonicalIDMap.put("twtpe", "Asia/Taipei");
            tzCanonicalIDMap.put("ROC", "Asia/Taipei");
            tzCanonicalIDMap.put("utcw05", "Etc/GMT+5");
            tzCanonicalIDMap.put("EST", "Etc/GMT+5");
            tzCanonicalIDMap.put("utcw06", "Etc/GMT+6");
            tzCanonicalIDMap.put("fihel", "Europe/Helsinki");
            tzCanonicalIDMap.put("iedub", "Europe/Dublin");
            tzCanonicalIDMap.put("Eire", "Europe/Dublin");
            tzCanonicalIDMap.put("utcw07", "Etc/GMT+7");
            tzCanonicalIDMap.put("MST", "Etc/GMT+7");
            tzCanonicalIDMap.put("utcw08", "Etc/GMT+8");
            tzCanonicalIDMap.put("utcw09", "Etc/GMT+9");
            tzCanonicalIDMap.put("utcw01", "Etc/GMT+1");
            tzCanonicalIDMap.put("pgpom", "Pacific/Port_Moresby");
            tzCanonicalIDMap.put("utcw02", "Etc/GMT+2");
            tzCanonicalIDMap.put("utcw03", "Etc/GMT+3");
            tzCanonicalIDMap.put("utcw04", "Etc/GMT+4");
            tzCanonicalIDMap.put("bgsof", "Europe/Sofia");
            tzCanonicalIDMap.put("usknx", "America/Indiana/Knox");
            tzCanonicalIDMap.put("America/Knox_IN", "America/Indiana/Knox");
            tzCanonicalIDMap.put("US/Indiana-Starke", "America/Indiana/Knox");
            tzCanonicalIDMap.put("bsnas", "America/Nassau");
            tzCanonicalIDMap.put("clipc", "Pacific/Easter");
            tzCanonicalIDMap.put("Chile/EasterIsland", "Pacific/Easter");
            tzCanonicalIDMap.put("clscl", "America/Santiago");
            tzCanonicalIDMap.put("Chile/Continental", "America/Santiago");
            tzCanonicalIDMap.put("aqmaw", "Antarctica/Mawson");
            tzCanonicalIDMap.put("isrey", "Atlantic/Reykjavik");
            tzCanonicalIDMap.put("Iceland", "Atlantic/Reykjavik");
            tzCanonicalIDMap.put("fotho", "Atlantic/Faeroe");
            tzCanonicalIDMap.put("Atlantic/Faroe", "Atlantic/Faeroe");
            tzCanonicalIDMap.put("ncnou", "Pacific/Noumea");
            tzCanonicalIDMap.put("idpnk", "Asia/Pontianak");
            tzCanonicalIDMap.put("nenim", "Africa/Niamey");
            tzCanonicalIDMap.put("cagoo", "America/Goose_Bay");
            tzCanonicalIDMap.put("mhkwa", "Pacific/Kwajalein");
            tzCanonicalIDMap.put("Kwajalein", "Pacific/Kwajalein");
            tzCanonicalIDMap.put("livdz", "Europe/Vaduz");
            tzCanonicalIDMap.put("ruuus", "Asia/Sakhalin");
            tzCanonicalIDMap.put("brmcz", "America/Maceio");
            tzCanonicalIDMap.put("zwhre", "Africa/Harare");
            tzCanonicalIDMap.put("careb", "America/Resolute");
            tzCanonicalIDMap.put("rukhndg", "Asia/Khandyga");
            tzCanonicalIDMap.put("careg", "America/Regina");
            tzCanonicalIDMap.put("Canada/East-Saskatchewan", "America/Regina");
            tzCanonicalIDMap.put("Canada/Saskatchewan", "America/Regina");
            tzCanonicalIDMap.put("rupkc", "Asia/Kamchatka");
            tzCanonicalIDMap.put("arsla", "America/Argentina/Salta");
            tzCanonicalIDMap.put("mykch", "Asia/Kuching");
            tzCanonicalIDMap.put("tjdyu", "Asia/Dushanbe");
            tzCanonicalIDMap.put("cafne", "America/Fort_Nelson");
            tzCanonicalIDMap.put("fjsuv", "Pacific/Fiji");
            tzCanonicalIDMap.put("pfppt", "Pacific/Tahiti");
            tzCanonicalIDMap.put("bibjm", "Africa/Bujumbura");
            tzCanonicalIDMap.put("aqmcm", "Antarctica/McMurdo");
            tzCanonicalIDMap.put("usmtm", "America/Metlakatla");
            tzCanonicalIDMap.put("uzskd", "Asia/Samarkand");
            tzCanonicalIDMap.put("cacfq", "America/Creston");
            tzCanonicalIDMap.put("ecgps", "Pacific/Galapagos");
            tzCanonicalIDMap.put("ruvvo", "Asia/Vladivostok");
            tzCanonicalIDMap.put("aqtrl", "Antarctica/Troll");
            tzCanonicalIDMap.put("usjnu", "America/Juneau");
            tzCanonicalIDMap.put("cawnp", "America/Winnipeg");
            tzCanonicalIDMap.put("Canada/Central", "America/Winnipeg");
            tzCanonicalIDMap.put("America/Rainy_River", "America/Winnipeg");
            tzCanonicalIDMap.put("kpfnj", "Asia/Pyongyang");
            tzCanonicalIDMap.put("kzkzo", "Asia/Qyzylorda");
            tzCanonicalIDMap.put("wsapw", "Pacific/Apia");
            tzCanonicalIDMap.put("uswsq", "America/Indiana/Petersburg");
            tzCanonicalIDMap.put("gncky", "Africa/Conakry");
            tzCanonicalIDMap.put("usxul", "America/North_Dakota/Beulah");
            tzCanonicalIDMap.put("rukgd", "Europe/Kaliningrad");
            tzCanonicalIDMap.put("tfpfr", "Indian/Kerguelen");
            tzCanonicalIDMap.put("utce10", "Etc/GMT-10");
            tzCanonicalIDMap.put("utce11", "Etc/GMT-11");
            tzCanonicalIDMap.put("mpspn", "Pacific/Saipan");
            tzCanonicalIDMap.put("utce12", "Etc/GMT-12");
            tzCanonicalIDMap.put("utce13", "Etc/GMT-13");
            tzCanonicalIDMap.put("arirj", "America/Argentina/La_Rioja");
            tzCanonicalIDMap.put("aumqi", "Antarctica/Macquarie");
            tzCanonicalIDMap.put("utce14", "Etc/GMT-14");
            tzCanonicalIDMap.put("ckrar", "Pacific/Rarotonga");
            tzCanonicalIDMap.put("arjuj", "America/Jujuy");
            tzCanonicalIDMap.put("America/Argentina/Jujuy", "America/Jujuy");
            tzCanonicalIDMap.put("aqvos", "Antarctica/Vostok");
            tzCanonicalIDMap.put("momfm", "Asia/Macau");
            tzCanonicalIDMap.put("Asia/Macao", "Asia/Macau");
            tzCanonicalIDMap.put("ussit", "America/Sitka");
            tzCanonicalIDMap.put("rudyr", "Asia/Anadyr");
            tzCanonicalIDMap.put("aolad", "Africa/Luanda");
            tzCanonicalIDMap.put("ruasf", "Europe/Astrakhan");
            tzCanonicalIDMap.put("adalv", "Europe/Andorra");
            tzCanonicalIDMap.put("rurtw", "Europe/Saratov");
            tzCanonicalIDMap.put("caedm", "America/Edmonton");
            tzCanonicalIDMap.put("Canada/Mountain", "America/Edmonton");
            tzCanonicalIDMap.put("America/Yellowknife", "America/Edmonton");
            tzCanonicalIDMap.put("utce07", "Etc/GMT-7");
            tzCanonicalIDMap.put("utce08", "Etc/GMT-8");
            tzCanonicalIDMap.put("utce09", "Etc/GMT-9");
            tzCanonicalIDMap.put("awaua", "America/Aruba");
            tzCanonicalIDMap.put("utce01", "Etc/GMT-1");
            tzCanonicalIDMap.put("utce02", "Etc/GMT-2");
            tzCanonicalIDMap.put("utce03", "Etc/GMT-3");
            tzCanonicalIDMap.put("utce04", "Etc/GMT-4");
            tzCanonicalIDMap.put("utce05", "Etc/GMT-5");
            tzCanonicalIDMap.put("utce06", "Etc/GMT-6");
            tzCanonicalIDMap.put("gfcay", "America/Cayenne");
            tzCanonicalIDMap.put("irthr", "Asia/Tehran");
            tzCanonicalIDMap.put("Iran", "Asia/Tehran");
            tzCanonicalIDMap.put("qadoh", "Asia/Qatar");
            tzCanonicalIDMap.put("usoea", "America/Indiana/Vincennes");
            tzCanonicalIDMap.put("ruoms", "Asia/Omsk");
            tzCanonicalIDMap.put("iqbgw", "Asia/Baghdad");
            tzCanonicalIDMap.put("fimhq", "Europe/Mariehamn");
            tzCanonicalIDMap.put("caglb", "America/Glace_Bay");
            tzCanonicalIDMap.put("czprg", "Europe/Prague");
            tzCanonicalIDMap.put("kzguw", "Asia/Atyrau");
            tzCanonicalIDMap.put("ruunera", "Asia/Ust-Nera");
            tzCanonicalIDMap.put("erasm", "Africa/Asmera");
            tzCanonicalIDMap.put("Africa/Asmara", "Africa/Asmera");
            tzCanonicalIDMap.put("pst8pdt", "PST8PDT");
            tzCanonicalIDMap.put("cvrai", "Atlantic/Cape_Verde");
            tzCanonicalIDMap.put("mgtnr", "Indian/Antananarivo");
            tzCanonicalIDMap.put("atvie", "Europe/Vienna");
            tzCanonicalIDMap.put("bnbwn", "Asia/Brunei");
            tzCanonicalIDMap.put("gpbbr", "America/Guadeloupe");
            tzCanonicalIDMap.put("srpbm", "America/Paramaribo");
            tzCanonicalIDMap.put("btthi", "Asia/Thimphu");
            tzCanonicalIDMap.put("Asia/Thimbu", "Asia/Thimphu");
            tzCanonicalIDMap.put("ancur", "America/Curacao");
            tzCanonicalIDMap.put("fmtkk", "Pacific/Truk");
            tzCanonicalIDMap.put("Pacific/Chuuk", "Pacific/Truk");
            tzCanonicalIDMap.put("Pacific/Yap", "Pacific/Truk");
            tzCanonicalIDMap.put("galbv", "Africa/Libreville");
            tzCanonicalIDMap.put("crsjo", "America/Costa_Rica");
            tzCanonicalIDMap.put("cst6cdt", "CST6CDT");
            tzCanonicalIDMap.put("ausyd", "Australia/Sydney");
            tzCanonicalIDMap.put("Australia/ACT", "Australia/Sydney");
            tzCanonicalIDMap.put("Australia/Canberra", "Australia/Sydney");
            tzCanonicalIDMap.put("Australia/NSW", "Australia/Sydney");
            tzCanonicalIDMap.put("kzura", "Asia/Oral");
            tzCanonicalIDMap.put("krsel", "Asia/Seoul");
            tzCanonicalIDMap.put("ROK", "Asia/Seoul");
            tzCanonicalIDMap.put("nfnlk", "Pacific/Norfolk");
            tzCanonicalIDMap.put("rumow", "Europe/Moscow");
            tzCanonicalIDMap.put("W-SU", "Europe/Moscow");
            tzCanonicalIDMap.put("skbts", "Europe/Bratislava");
            tzCanonicalIDMap.put("gpsbh", "America/St_Barthelemy");
            tzCanonicalIDMap.put("scmaw", "Indian/Mahe");
            tzCanonicalIDMap.put("tdndj", "Africa/Ndjamena");
            tzCanonicalIDMap.put("bhbah", "Asia/Bahrain");
            tzCanonicalIDMap.put("mcmon", "Europe/Monaco");
            tzCanonicalIDMap.put("hebron", "Asia/Hebron");
            tzCanonicalIDMap.put("mxpvr", "America/Bahia_Banderas");
            tzCanonicalIDMap.put("gmbjl", "Africa/Banjul");
            tzCanonicalIDMap.put("brbel", "America/Belem");
            tzCanonicalIDMap.put("kzakx", "Asia/Aqtobe");
            tzCanonicalIDMap.put("brcgr", "America/Campo_Grande");
            tzCanonicalIDMap.put("cavan", "America/Vancouver");
            tzCanonicalIDMap.put("Canada/Pacific", "America/Vancouver");
            tzCanonicalIDMap.put("vgtov", "America/Tortola");
            tzCanonicalIDMap.put("clpuq", "America/Punta_Arenas");
            tzCanonicalIDMap.put("usndcnt", "America/North_Dakota/Center");
            tzCanonicalIDMap.put("phmnl", "Asia/Manila");
            tzCanonicalIDMap.put("idmak", "Asia/Makassar");
            tzCanonicalIDMap.put("Asia/Ujung_Pandang", "Asia/Makassar");
            tzCanonicalIDMap.put("kenbo", "Africa/Nairobi");
            tzCanonicalIDMap.put("brcgb", "America/Cuiaba");
            tzCanonicalIDMap.put("hubud", "Europe/Budapest");
            tzCanonicalIDMap.put("amevn", "Asia/Yerevan");
            tzCanonicalIDMap.put("gdgnd", "America/Grenada");
            tzCanonicalIDMap.put("arluq", "America/Argentina/San_Luis");
            tzCanonicalIDMap.put("ruyek", "Asia/Yekaterinburg");
            tzCanonicalIDMap.put("tmasb", "Asia/Ashgabat");
            tzCanonicalIDMap.put("Asia/Ashkhabad", "Asia/Ashgabat");
            tzCanonicalIDMap.put("usaeg", "America/Indiana/Marengo");
            tzCanonicalIDMap.put("tntun", "Africa/Tunis");
            tzCanonicalIDMap.put("runoz", "Asia/Novokuznetsk");
            tzCanonicalIDMap.put("lulux", "Europe/Luxembourg");
            tzCanonicalIDMap.put("mdkiv", "Europe/Chisinau");
            tzCanonicalIDMap.put("Europe/Tiraspol", "Europe/Chisinau");
            tzCanonicalIDMap.put("trist", "Europe/Istanbul");
            tzCanonicalIDMap.put("Asia/Istanbul", "Europe/Istanbul");
            tzCanonicalIDMap.put("Turkey", "Europe/Istanbul");
            tzCanonicalIDMap.put("nuiue", "Pacific/Niue");
            tzCanonicalIDMap.put("brrec", "America/Recife");
            tzCanonicalIDMap.put("kzala", "Asia/Almaty");
            tzCanonicalIDMap.put("globy", "America/Scoresbysund");
            tzCanonicalIDMap.put("debsngn", "Europe/Busingen");
            tzCanonicalIDMap.put("uschi", "America/Chicago");
            tzCanonicalIDMap.put("US/Central", "America/Chicago");
            tzCanonicalIDMap.put("vuvli", "Pacific/Efate");
            tzCanonicalIDMap.put("brfor", "America/Fortaleza");
            tzCanonicalIDMap.put("cxxch", "Indian/Christmas");
            tzCanonicalIDMap.put("noosl", "Europe/Oslo");
            tzCanonicalIDMap.put("ciabj", "Africa/Abidjan");
            tzCanonicalIDMap.put("bwgbe", "Africa/Gaborone");
            tzCanonicalIDMap.put("mhmaj", "Pacific/Majuro");
            tzCanonicalIDMap.put("macas", "Africa/Casablanca");
            tzCanonicalIDMap.put("usadk", "America/Adak");
            tzCanonicalIDMap.put("America/Atka", "America/Adak");
            tzCanonicalIDMap.put("US/Aleutian", "America/Adak");
            tzCanonicalIDMap.put("saruh", "Asia/Riyadh");
            tzCanonicalIDMap.put("usinvev", "America/Indiana/Vevay");
            tzCanonicalIDMap.put("gggci", "Europe/Guernsey");
            tzCanonicalIDMap.put("rereu", "Indian/Reunion");
            tzCanonicalIDMap.put("getbs", "Asia/Tbilisi");
            tzCanonicalIDMap.put("umawk", "Pacific/Wake");

            likelyScriptMap.put("Mand", " mid myz ");
            likelyScriptMap.put("Ahom", " aho ");
            likelyScriptMap.put("Mani", " xmn ");
            likelyScriptMap.put("Khar", " pgd pra ");
            likelyScriptMap.put("Mroo", " mro ");
            likelyScriptMap.put("Runr", " non ");
            likelyScriptMap.put("Tnsa", " nst ");
            likelyScriptMap.put("Geor", " bbl jge ka oav sva xmf ");
            likelyScriptMap.put("Ugar", " uga ");
            likelyScriptMap.put("Nkoo", " nqo ");
            likelyScriptMap.put("Goth", " got ");
            likelyScriptMap.put("Avst", " ae ");
            likelyScriptMap.put("Cham", " cjm ");
            likelyScriptMap.put("Kits", " zkt ");
            likelyScriptMap.put("Kali", " eky kvy kyu ");
            likelyScriptMap.put("Mymr", " aio blk bwe csh dnu dnv hpo int jkm kht kjp ksu ksw kvq kvt kxf kxk mnw mwt my obr pce phk pll pwo pyx rbb rki rmz shn tco tjl tvn ");
            likelyScriptMap.put("Gran", " oty ");
            likelyScriptMap.put("Plrd", " hmd ktp lpo sfm ygp yna ysy ywq ywu ");
            likelyScriptMap.put("Lepc", " lep ");
            likelyScriptMap.put("Bamu", " bax ");
            likelyScriptMap.put("Orya", " bdv bfw dso dwk gaq gbj gdb jun juy or peg rei spv uki xis ");
            likelyScriptMap.put("Cyrl", " ab ady agx akv alr alt ani aqc atv av ba be bg bhh bph bua bxm ce chm cji ckt clw crh cu cv dar ddo dlg dng enf enh eve evn gdo gin gld huz inh itl jct jdt kaa kap kbd kca ket khv kim kjh kk koi kpt kpy krc krk kum kv kva ky lbe lez mdf mk mn mns mrj mtm mud myv ndf neg nio niv nog oaa oac omk orv os paq rsk ru rue rut sah sel sgh sia sjd sjt sr sty tab tg tin tt tyv ude udm ugh uk ulc xal xas xdq xpm xrm xrn xwo yai ykg ynk yrk ysr yug yux zko zkz ");
            likelyScriptMap.put("Java", " osi tes ");
            likelyScriptMap.put("Narb", " xna ");
            likelyScriptMap.put("Egyp", " egy ");
            likelyScriptMap.put("Pauc", " ctd ");
            likelyScriptMap.put("Sogd", " sog ");
            likelyScriptMap.put("Cakm", " ccp tnv ");
            likelyScriptMap.put("Toto", " txo ");
            likelyScriptMap.put("Beng", " aot as bn bpy cdz ctg der grt kdq mni rah rkt syl unr unx ");
            likelyScriptMap.put("Yiii", " ii nos nsd nsf nsv nty smh yig yiv ysd ysn ysp ");
            likelyScriptMap.put("Sinh", " pi si ");
            likelyScriptMap.put("Kana", " ain ryu ");
            likelyScriptMap.put("Sora", " srb ");
            likelyScriptMap.put("Mtei", " omp ");
            likelyScriptMap.put("Tibt", " adx bfu bo cgk cna dka dre dz dzl ght goe jda jya khg kkf lbj lkh luk lya muk neh npb ole otb sbu sgt sip spt tcn tgf tsj zau ");
            likelyScriptMap.put("Armi", " arc ");
            likelyScriptMap.put("Hmnp", " hnj mww ");
            likelyScriptMap.put("Thai", " bzi cbn cog kdt khf kjt kxm lcp lwl lwm mkm mlf mpz mra nyl pht prt pww skb sou th thm tts ugo urk yoy ");
            likelyScriptMap.put("Lisu", " lbc lis ");
            likelyScriptMap.put("Armn", " axm hy hyw rmi ");
            likelyScriptMap.put("Aghb", " udi ");
            likelyScriptMap.put("Thaa", " dv ");
            likelyScriptMap.put("Takr", " bht hii jna scu ");
            likelyScriptMap.put("Kore", " ko ");
            likelyScriptMap.put("Nshu", " zhx ");
            likelyScriptMap.put("Ital", " ett osc pgn xve ");
            likelyScriptMap.put("Chrs", " xco ");
            likelyScriptMap.put("Saur", " saz ");
            likelyScriptMap.put("Copt", " cop ");
            likelyScriptMap.put("Cprt", " ecy grc ");
            likelyScriptMap.put("Brah", " pka pmh ");
            likelyScriptMap.put("Orkh", " otk ");
            likelyScriptMap.put("Hans", " cdo cjy cnp csp czh gan hak hsn lzh nan wuu zh ");
            likelyScriptMap.put("Hant", " yue ");
            likelyScriptMap.put("Hang", " jje okm ");
            likelyScriptMap.put("Phli", " pal ");
            likelyScriptMap.put("Hani", " how oko swi zch zeh zgb zgm zgn zhd zlj zln zlq zqe zyg zyn zzj ");
            likelyScriptMap.put("Samr", " sam smp ");
            likelyScriptMap.put("Soyo", " cmg ");
            likelyScriptMap.put("Wcho", " nnp ");
            likelyScriptMap.put("Merc", " xmr ");
            likelyScriptMap.put("Taml", " bfq ctt era iru kfe kfi muv ta tcx vaa xub xuj ");
            likelyScriptMap.put("Khmr", " brb km krr krv okz pcb rka smu tpu xhm ");
            likelyScriptMap.put("Xsux", " akk hit htx nei oht ");
            likelyScriptMap.put("Newa", " nwc ");
            likelyScriptMap.put("Phnx", " obm phn ");
            likelyScriptMap.put("Osge", " osa ");
            likelyScriptMap.put("Marc", " xzh ");
            likelyScriptMap.put("Cher", " chr ");
            likelyScriptMap.put("Mong", " bxu mvf ");
            likelyScriptMap.put("Bass", " bsq ");
            likelyScriptMap.put("Grek", " aat bgx cpg ecr el pnt scx tsd uum xpg yej ");
            likelyScriptMap.put("Lydi", " xld ");
            likelyScriptMap.put("Latn", " aa aaa aab aac aad aae aag aah aai aak aal aan aap aaq aas aau aaw aax aaz aba abb abc abd abe abf abg abi abm abn abo abp abr abs abt abu abw abx aby abz aca acb acd ace acf ach acn acp acr acs act acu acv acy acz ada adb add ade adg adh adi adj adl adn ado adq adr adt adu adw adz aea aek ael aem aer aeu aew aey aez af afd afe afh afi afk afn afo afp afs afu afz aga agb agc agd age agf agg agh agk agl agm agn ago agq agr ags agt agu agv agw agy agz aha ahb ahh ahi ahk ahl ahm ahn ahp ahs aht aia aic aid aie aif aig aik ail aim aip air ait aiw aix aiy aja ajg aji ajn ajw ajz ak akb akc akd ake akf akg akh aki akl ako akp akq akr aks akt aku akw akz ala alc ald ale alf alh ali alj alm aln alo alp alq alu alx aly alz ama amb amc ame amf amg ami amj amk amm amn amo amp amq amr amt amu amv amx amy amz an ana anb anc and ane anf ang anh anj ank anl anm ann ano ans ant anv anw anx any anz aoa aob aoc aod aoe aof aog aoi aoj aok aol aom aon aor aos aox aoz apb ape apf apg api apj apk apl apm apn apo app apr aps apt apu apv apw apx apy apz aqd aqg aqk aqm aqn aqr aqt aqz ard are arh ari arj ark arl arn aro arp arr aru arw arx asa asb asc asg ash asi asj asl asn aso ass ast asu asv asx asy asz ata atb atc atd ate atg ati atj atk atl atm ato atp atq atr ats att atu atw atx aty atz aua auc aud aug auh aui auk aul aum aun auo aup auq aur aut auu auw auy avb avi avk avm avn avo avs avt avu avv awb awc awe awg awh awi awk awm awo awr aws awt awu awv aww awx awy axb axe axg axk axl axx ay aya ayb ayc ayd aye ayg ayi ayk ayo ayq ays ayt ayu ayz az azd azg azm azn azo azt azz baa bab bac bae baf bag bah baj ban bao bar bas bau bav baw bay bba bbb bbc bbd bbe bbf bbg bbi bbj bbk bbm bbn bbo bbp bbq bbr bbs bbt bbu bbv bbw bbx bby bca bcb bcd bce bcf bcg bch bci bcj bck bcm bcn bco bcp bcr bcs bct bcu bcv bcw bcy bcz bda bdb bdc bdd bde bdf bdg bdh bdi bdj bdk bdl bdm bdn bdo bdp bdq bdr bds bdt bdu bdw bdx bdy bea beb bec bed bef beh bei bek bem beo bep beq bes bet beu bev bew bex bey bez bfa bfc bfd bfe bff bfg bfh bfj bfl bfm bfn bfo bfp bfs bfx bga bgb bgf bgg bgi bgj bgo bgr bgs bgt bgu bgv bgy bgz bhc bhf bhg bhl bhp bhq bhr bhs bhv bhw bhy bhz bi bia bib bid bie bif big bik bil bim bin bio bip biq bir bit biu biv biw biz bja bjb bjc bjg bjh bji bjk bjl bjn bjo bjp bjr bjs bjt bju bjv bjw bjx bjy bjz bka bkc bkd bkf bkg bkh bki bkj bkl bkm bkn bko bkp bkq bkr bks bkt bku bkv bkw bkx bky bkz bla blb blc bld ble blf blh bli blj blm bln blo blp blq blr bls blv blw blx bly blz bm bma bmb bmc bmd bme bmf bmg bmh bmi bmk bml bmm bmn bmo bmp bmq bmr bms bmu bmv bmw bmx bmz bna bnb bnc bnd bne bnf bng bni bnj bnk bnm bnn bno bnp bnq bnr bnu bnv bnw bnx bny bnz boa bob boe bof boh boj bok bol bom bon boo bop boq bor bot bou bov bow box boy boz bpa bpc bpd bpe bpg bpi bpj bpk bpl bpm bpo bpp bpq bpr bps bpt bpu bpv bpw bpz bqa bqb bqc bqd bqf bqg bqj bqk bql bqm bqo bqp bqq bqr bqs bqt bqu bqv bqw bqx bqz br brc brf brg bri brj brl brm brn brp brq brr brs brt bru bry brz bs bsa bsb bsc bse bsf bsi bsj bsl bsm bsn bso bsp bsr bss bsu bsv bsw bsx bsy bta btc bte btf btg bth bti btj btn bto btp btq btr bts btt btu btw btx bty btz bub buc bud bue buf bug buh bui buj buk bum bun buo bup buq bus but buu buv buw bux buy buz bva bvb bvc bvd bve bvf bvg bvh bvi bvj bvk bvm bvn bvo bvq bvr bvt bvu bvv bvw bvx bvy bvz bwa bwb bwc bwd bwf bwg bwh bwi bwj bwk bwl bwm bwo bwp bwq bwr bws bwt bwu bww bwx bwy bwz bxa bxb bxc bxf bxg bxh bxi bxj bxl bxn bxo bxp bxq bxs bxv bxw bxz bya byb byc byd bye byf byi byj byk byl bym byp byr bys byv byx byz bza bzb bzc bzd bze bzf bzh bzj bzk bzl bzm bzn bzo bzp bzq bzr bzt bzu bzv bzw bzx bzy bzz ca caa cab cac cad cae caf cag cah caj cak cal cam can cao cap caq car cas cav caw cax cay caz cbb cbc cbd cbg cbi cbj cbk cbl cbo cbq cbr cbs cbt cbu cbv cbw cby ccc ccd cce ccg cch ccj ccl ccm cco ccr cdf cdr cea ceb ceg cek cen cet cey cfa cfd cfg cfm cga cgc cgg ch chb chd chf chh chj chk chl chn cho chp chq cht chw chy chz cia cib cic cie cim cin cip cir ciw ciy cje cjh cjk cjn cjo cjp cjs cjv ckl ckm ckn cko ckq ckr cks cku ckv ckx cky ckz cla clc cle cli clj clk cll clm clo clt clu cly cma cme cmi cml cmo cmr cms cmt cnb cnc cng cnh cni cnk cnl cnq cns cnt cnw cnx co coa cob coc cod coe cof coh coj cok col com coo coq cot cou cox coz cpa cpb cpc cpi cpn cpo cps cpu cpx cpy cqd crb crc crd crf crg cri crn cro crq crs crt crv crw crx cry crz cs csa csb csj csk csm cso css cst csv csy csz cta ctc cte cth ctl ctm cto ctp cts ctu ctz cua cub cuc cuh cui cuj cuk cul cuo cup cut cuv cux cvg cvn cwa cwb cwe cwg cwt cy cya cyb cyo czn czt da daa dac dad dae dag dah dai daj dak dal dam dao das dau dav daw dax daz dba dbb dbd dbe dbf dbg dbi dbj dbl dbm dbn dbo dbp dbq dbt dbu dbv dbw dby dcr dda ddd dde ddg ddi ddj ddn ddr dds ddw de dec ded dee deg dei dek del dem den deq des dev dez dga dgb dgc dgd dge dgg dgh dgi dgk dgn dgr dgs dgt dgw dgx dgz dhg dhl dhm dhr dhs dhu dhv dhx dia dib dic did dif dig dih dii dij dil din dio dip dir dis diu diw dix diy diz dja djb djc djd dje djf dji djj djk djm djn djo djr dju djw dkg dkk dkr dks dkx dlm dln dma dmb dmc dmd dme dmg dmm dmo dmr dms dmu dmv dmw dmx dmy dna dnd dne dni dnj dnk dnn dno dnr dnt dnw dny doa dob doc doe dof doh dok dol don doo dop dor dos dot dov dow doy dpp drc drg dri drl drn dro drt dru dsb dsh dsi dsn dsq dta dtb dtd dth dti dtk dtm dto dtp dtr dts dtt dtu dua duc due duf dug dui duk dul dum dun duo dup duq dur duu duv duw dux duy duz dva dwa dwr dws dwu dww dwy dya dyb dyd dyg dyi dym dyn dyo dyu dyy dza dze dzg dzn eaa ebc ebg ebk ebo ebr ebu ee efa efe efi ega egl egm ego ehu eip eit eiv eja eka eke ekg eki ekl ekm eko ekp ekr ele elk elm elo elu ema emb eme emi emm emn emp ems emw emx emz en ena enb enc end enl enm enn eno enq enr env enw enx eo eot epi erg erh eri erk err ert erw es ese esi esm ess esu esy et etb etn eto etr ets etu etx etz eu evh ewo ext eya eyo eza eze faa fab fad faf fag fah fai faj fak fal fam fan fap far fau fax fbl fer ff ffi ffm fgr fi fie fif fil fip fir fit fiw fj fkk fkv fla flh fli fll fln flr fly fmp fnb fng fni fo fod foi fom fon for fos fpe fqs fr frc frd frk frm fro frp frq frr frs frt fud fue fuf fuh fui fum fun fuq fur fut fuu fuv fuy fvr fwa fwe fy ga gaa gab gac gad gae gaf gag gah gai gaj gak gal gam gao gap gar gat gaw gax gay gba gbb gbd gbe gbf gbg gbh gbi gbn gbp gbq gbr gbs gbu gbv gbw gbx gby gcc gcd gcf gcl gcn gcr gct gd gdc gdd gde gdf gdg gdh gdi gdj gdk gdl gdm gdn gdq gdr gdt gdu gea geb gec ged gef geg geh gei gej gek gel geq ges gev gew gex gey gfk gga ggb ggd gge ggk ggl ggt ggu ggw ghc ghk ghn ghs gia gib gic gid gie gih gil gim gip giq gir gis git gix giy giz gjm gjn gjr gka gkd gke gkn gko gkp gku gl glb glc glj gll glo glr glu glw gma gmb gmd gmg gmh gmm gmn gmr gmu gmx gmz gn gna gnb gnc gnd gne gng gnh gni gnj gnk gnl gnm gnn gnq gnr gnt gnu gnw gnz goa gob goc god gog goh goi gol goo gop goq gor gos gou gov gow gox goy gpa gpe gpn gqa gqn gqr grb grd grg grh gri grj grm grq grs grv grw grx gry grz gsl gsn gso gsp gsw gta gtu gua gub guc gud gue guf guh gui guk gul gum gun guo gup guq gur gut guu guw gux guz gv gva gvc gve gvf gvj gvl gvm gvn gvo gvp gvs gvy gwa gwb gwd gwe gwg gwi gwj gwm gwn gwr gwu gww gwx gxx gyb gyd gye gyf gyg gyi gyl gym gyn gyr gyy gyz gza gzn ha haa had hae hag hah hai haj hal ham han hao hap haq has hav haw hax hay hba hbb hbn hbu hch hed heg heh hei hem hgm hgw hhi hhr hhy hia hib hid hif hig hih hij hik hil hio hir hiw hix hji hka hke hkk hla hld hlt hma hmb hmf hmm hmn hmp hmr hms hmt hmu hmv hmw hmy hmz hna hng hnh hni hnn hns ho hoa hob hod hoe hoi hol hom hoo hop hor hot hov hr hra hrc hre hrk hrm hro hrp hru hrw hrx hsb ht hti hto hts htu hu hub huc hud hue huf hug huh hui huk hul hum hup hur hus huu huv huw hux hvc hve hvk hvn hvv hwa hwc hwo hya hz ia iai ian iar iba ibb ibd ibe ibg ibh ibl ibm ibn ibr ibu iby ica ich icr id ida idb idc idd ide idi idr ids idt idu ie ifa ifb ife iff ifk ifm ifu ify ig igb ige igg igl igm ign igo igs igw ihb ihi ihp ihw iin ijc ije ijj ijn ijs ik iki ikk ikl iko ikp ikr ikt ikv ikw ikx ikz ila ilb ilg ili ilk ilm ilo ilp ilu ilv imi iml imn imo imr ims imt in inb ing inj inn ino inp io iou iow ipi ipo iqu iqw ire irh iri irk irn irx iry is isa isc isd ish isi ism isn iso ist isu it itb itd ite iti itm ito itr its itt itv itw itx ity itz ium ivb ivv iwk iwm iwo iws ixc ixl iya iyo iyx izh izr izz jaa jab jac jae jaf jah jaj jak jal jam jan jao jaq jas jau jax jay jaz jbi jbj jbk jbm jbo jbr jbt jbu jbw jeb jeh jei jek jel jen jer jet jeu jgb jgk jgo jhi jia jib jic jid jie jig jil jim jit jiu jiv jiy jjr jka jko jku jle jma jmb jmc jmd jmi jmn jmr jms jmw jmx jng jni jnj job jod jor jow jqr jra jrr jrt jru jua jub jud juh jui juk jum juo jup jur jut juu juw jv jvd jvn jw jwi jyy kab kac kad kag kah kai kaj kak kam kao kaq kav kax kay kba kbb kbc kbe kbh kbi kbj kbk kbl kbm kbn kbo kbp kbq kbr kbs kbt kbv kbw kbx kbz kcb kcc kcd kce kcf kcg kch kci kcj kck kcl kcm kcn kco kcp kcq kcs kct kcu kcv kcw kcz kda kdc kdd kde kdf kdg kdh kdi kdj kdk kdl kdm kdn kdp kdr kdw kdx kdy kdz kea keb kec ked kee kef keg keh kei kek kel kem ken keo ker kes keu kew kez kff kfl kfn kfo kfv kfw kfz kg kga kgb kge kgf kgk kgl kgo kgp kgq kgr kgs kgt kgu kgv kgw kgx kha khc khd khe khh khj khl khp khq khr khs khu khx khy khz ki kia kib kic kid kie kig kih kij kil kio kiq kis kit kiu kiv kiw kix kiy kiz kj kja kjb kjc kjd kje kji kjj kjk kjm kjn kjq kjr kjs kju kjx kjy kka kkb kkc kkd kke kkg kki kkj kkk kkl kkm kko kkp kkq kkr kks kku kkv kkw kkx kky kkz kl kla klb klc kld klf klg klh kli klk kll klm kln klo klp klq kls klt klu klv klw klx kly klz kma kmb kmc kmd kme kmf kmg kmh kmi kmk kml kmm kmn kmo kmp kmq kms kmt kmu kmv kmw kmx kmy kna knb knd kne knf kni knj knk knl knm kno knp knq knr kns knt knu knv knw knx kny knz koa koc kod koe kof kog koh kol koo kop koq kos kot kou kov kow koy koz kpa kpc kpd kpe kpf kpg kph kpi kpj kpk kpl kpm kpn kpo kpq kpr kps kpu kpw kpx kpz kqa kqb kqc kqe kqf kqg kqh kqi kqj kqk kql kqm kqn kqo kqp kqq kqr kqs kqt kqu kqv kqw kqx kqz kr krb krd kre krf krh kri krj krl krn krp krs krt krw krx kry krz ksb ksc ksd kse ksf ksg ksh ksi ksj ksk ksl ksm ksn kso ksp ksq ksr kss kst ksv ksx kta ktc ktd ktf ktg kth kti ktj ktk ktm ktn kto ktq kts ktt ktu ktv ktw ktx kty ktz ku kub kuc kud kue kug kuh kui kuj kuk kul kun kuo kup kuq kus kut kuu kuv kuw kux kuy kuz kvb kvc kvd kve kvf kvg kvh kvi kvj kvl kvm kvn kvo kvp kvr kvv kvw kvz kw kwa kwb kwc kwd kwe kwf kwg kwh kwi kwj kwk kwl kwm kwn kwo kwp kwr kws kwt kwu kwv kww kwy kwz kxa kxb kxc kxd kxi kxj kxn kxo kxq kxr kxt kxv kxw kxx kxy kxz kya kyb kyc kyd kye kyf kyg kyh kyi kyj kyk kyl kym kyn kyo kyq kyr kys kyt kyx kyy kyz kza kzb kzc kzd kze kzf kzi kzk kzl kzm kzn kzo kzp kzr kzs kzu kzv kzw kzx kzy kzz la laa lac lag lai laj lal lam lan lap laq lar las lau law lax laz lb lbb lbi lbl lbn lbq lbt lbu lbv lbw lbx lby lbz lcc lcd lce lcf lch lcl lcm lcq lcs lda ldb ldd ldg ldh ldi ldj ldk ldl ldm ldn ldo ldp ldq lea leb lec led lee lef leh lei lej lek lel lem len leo leq ler les let leu lev lew lex ley lfa lfn lg lga lgb lgg lgh lgi lgk lgl lgm lgn lgo lgq lgr lgt lgu lgz lha lhh lhi lhn lht lhu li lia lib lic lid lie lig lih lij lik lil lio lip liq lir liu liv liw lix liy liz lja lje lji ljl ljp ljw ljx lka lkb lkc lkd lke lkj lkl lkm lkn lko lkr lks lkt lku lky lla llb llc lld lle llf llg lli llj llk lll llm lln llp llq llu llx lma lmb lmc lmd lme lmf lmg lmi lmj lmk lml lmo lmp lmq lmr lmu lmv lmw lmx lmy ln lna lnb lnd lnh lni lnj lnl lnm lnn lns lnu lnw lnz loa lob loc loe log loh loi loj lok lol lom lon loo lop loq lor los lot lou low lox loz lpa lpe lpn lpx lqr lra lrg lri lrm lrn lro lrt lrv lrz lse lsi lsm lsr lt ltg lth lti ltn lto lts ltu lu lua luc lud lue luf lui luj lul lum lun luo lup luq lur lus lut luw luy lv lva lvi lvk lvu lwa lwe lwg lwh lwo lwt lww lxm lyn lzl lzn lzz maa mab mad mae maf maj mak mam man maq mas mat mau mav maw max maz mba mbb mbc mbd mbf mbh mbi mbj mbk mbl mbm mbn mbo mbp mbq mbr mbs mbt mbu mbv mbw mbx mbz mca mcb mcc mcd mce mcf mcg mch mci mcj mck mcl mcm mcn mco mcp mcq mcr mcs mct mcu mcv mcw mcx mcy mcz mda mdb mdc mdd mdg mdh mdi mdj mdk mdm mdn mdp mdq mdr mds mdt mdu mdv mdw mdz mea meb mec med mee meh mej mek mel mem men meo mep meq mer mes met meu mev mew mey mez mfb mfc mfd mfe mff mfg mfh mfj mfk mfl mfm mfn mfo mfp mfq mfr mft mfu mfv mfw mfx mfy mfz mg mgb mgc mgd mge mgf mgg mgh mgi mgj mgk mgl mgm mgn mgo mgq mgr mgs mgt mgu mgv mgw mgy mgz mh mhb mhc mhd mhe mhf mhg mhi mhk mhl mhm mhn mho mhp mhq mhs mht mhu mhw mhx mhy mhz mi mia mib mic mie mif mig mih mii mij mik mil mim min mio mip miq mir mit miu miw mix miy miz mjb mjc mjd mje mjg mjh mji mjj mjk mjm mjn mjs mjw mjx mjy mka mkc mkf mkj mkk mkl mkn mko mkp mkr mks mkt mku mkv mkw mkx mky mkz mla mlb mlc mle mlh mli mlj mlk mll mln mlo mlp mlq mlr mls mlu mlv mlw mlx mlz mma mmb mmc mmd mme mmf mmg mmh mmi mmm mmn mmo mmp mmq mmr mmt mmu mmv mmw mmx mmy mmz mna mnb mnd mne mnf mng mnh mnl mnm mnn mnp mnq mnr mnu mnv mnx mny mnz mo moa moc mod moe mog moh moi moj mok mom moo mop moq mor mos mot mou mov mow mox moy moz mpa mpb mpc mpd mpe mpg mph mpi mpj mpk mpl mpm mpn mpo mpp mpq mpr mps mpt mpu mpv mpw mpx mpy mqa mqb mqc mqe mqf mqg mqh mqi mqj mqk mql mqm mqn mqo mqp mqq mqr mqs mqu mqv mqw mqx mqy mqz mrb mrc mrf mrg mrh mrk mrl mrm mrn mrp mrq mrs mrt mru mrv mrw mrx mry mrz ms msb msc mse msf msg msh msi msj msk msl msm msn mso msp msq mss msu msv msw msx msy msz mt mta mtb mtc mtd mte mtf mtg mth mti mtj mtk mtl mtn mto mtp mtq mts mtt mtu mtv mtw mtx mty mua mub muc mue mug muh mui muj mum muo muq mur mus muu mux muy mva mvd mvg mvh mvk mvl mvn mvo mvp mvq mvr mvs mvt mvu mvv mvw mvx mwa mwb mwc mwe mwf mwg mwh mwi mwk mwl mwm mwn mwo mwp mwq mws mwu mwv mwz mxa mxb mxc mxd mxe mxf mxg mxh mxi mxj mxk mxl mxm mxn mxo mxp mxq mxr mxs mxt mxu mxv mxw mxx mxy mxz myb myc mye myf myg myh myj myk myl myp myr myu myw myx myy mza mzd mze mzh mzi mzj mzk mzl mzm mzo mzp mzq mzr mzt mzu mzv mzw mzx mzz na naa nab nac nae naf nag naj nak nal nam nap naq nar nas nat naw nax nay naz nb nba nbb nbc nbd nbe nbh nbi nbj nbk nbm nbn nbo nbp nbq nbr nbt nbu nbv nbw nby nca ncb ncc nce ncf ncg nch nci ncj nck ncl ncm ncn nco ncr nct ncu ncx ncz nd nda ndb ndc ndd ndg ndh ndi ndj ndk ndl ndm ndn ndp ndq ndr nds ndt ndu ndv ndw ndx ndy ndz nea neb nec ned nee nej nek nem nen neo neq ner net neu nex ney nez nfa nfd nfl nfr nfu ng nga ngb ngc ngd nge ngg ngh ngi ngj ngk ngl ngm ngn ngp ngq ngr ngs ngu ngv ngw ngx ngy ngz nha nhb nhc nhd nhe nhf nhg nhi nhk nhm nhn nho nhp nhq nhr nht nhu nhv nhw nhx nhy nhz nia nib nid nie nif nig nih nii nij nil nim nin niq nir nis niu niw nix niy niz nja njb njd njh nji njj njl njm njn njo njr njs njt nju njx njy njz nka nkb nkc nkd nke nkf nkg nkh nki nkj nkk nkm nkn nko nkq nkr nks nkt nku nkv nkw nkx nkz nl nla nlc nle nlg nlj nlk nlo nlq nlu nlv nlw nly nlz nma nmb nmc nmd nme nmf nmg nmh nmi nmj nmk nml nmn nmo nmp nmq nmr nms nmt nmu nmv nmw nmx nmz nn nna nnb nnc nnd nne nnf nng nnh nni nnj nnk nnl nnm nnn nnq nnr nnt nnu nnv nnw nny nnz no noa noc nof noh noj nok nop noq not nou nov now noy npg nph npl npn npo nps npu npx npy nqg nqk nql nqm nqn nqq nqt nqy nr nra nrb nre nrf nrg nri nrk nrl nrm nrp nru nrx nrz nsa nsb nsc nse nsg nsh nsm nsn nso nsq nss nsu nsw nsx nsy nsz ntd nte ntg nti ntj ntk ntm nto ntp ntr ntu ntx nua nuc nud nue nuf nug nuh nui nuj nuk num nun nuo nup nuq nur nus nut nuu nuv nuw nux nuy nuz nv nvh nvm nvo nwb nwe nwg nwi nwm nwo nwr nww nxa nxd nxe nxg nxi nxl nxn nxo nxq nxr nxx ny nyb nyc nyd nye nyf nyg nyh nyi nyj nyk nym nyn nyo nyp nyr nys nyt nyu nyv nyx nyy nza nzb nzd nzi nzk nzm nzu nzy nzz obi obk obl obo obt obu oc oca oco ocu oda odt odu ofu ogb ogc ogg ogo ogu oia oie oin ojb ojc ojv ojw oka okb okc okd oke okg oki okk okr oks oku okv okx old olk olm olo olr olt olu om oma omb omc omg omi oml omo omt omu omw ona one ong oni onj onk onn ono onp onr ons ont onu onx ood oor opa opk opm opo opt opy ora orc ore org orn oro orr ors orw orx orz oso osp ost osu osx otd ote oti otl otm otn otq otr ots ott otu otw otx otz oub oue oum ovd owi owl oyd oym oyy ozm pab pac pad pae paf pag pah pai pak pam pao pap par pas pau pav paw pax pay paz pbb pbc pbe pbf pbg pbh pbi pbl pbm pbn pbo pbp pbr pbs pbv pby pca pcc pcd pck pcm pcn pcp pcw pda pdc pdn pdo pdt pdu pea peb ped pee pei pek pel pem pep peq pev pex pey pez pfa pfe pfl pga pgi pgk pgs pgu phg phh phm pia pib pic pid pif pig pih pij pil pim pin pio pip pir pis pit piu piv piw pix piy piz pjt pkb pkg pkh pkn pko pkp pku pl pla plb plc pld ple plg plh pln plo plr pls plu plv plw plz pma pmb pmd pme pmf pmi pmj pml pmm pmn pmo pmq pmr pms pmt pmw pmx pmy pmz pna pnc pnd pne png pnh pni pnj pnk pnl pnm pnn pno pnp pnq pnr pns pnv pnw pny pnz poc poe pof pog poh poi pok pom pon poo pop poq pos pot pov pow poy ppe ppi ppk ppl ppm ppn ppo ppp ppq pps ppt pqa pqm pre prf prg prh pri prk prm pro prq prr pru prw psa pse psm psn psq pss psw pt pta pth pti ptn pto ptp ptr ptt ptu ptv pua pub puc pud pue puf pug pui puj puo pup puq pur put puu puw pux puy pwa pwb pwg pwm pwn pxm pye pym pyn pyu pyy pze pzh pzn qu qua qub quc qud quf qug qui quk qul qum qun qup quq qur qus quv quw qux quy qva qvc qve qvh qvi qvj qvl qvm qvn qvo qvp qvs qvw qvz qwa qwc qwh qwm qws qwt qxa qxc qxh qxl qxn qxo qxp qxr qxt qxu qxw qya qyp rac rad rag rai rak ram ran rao rap rar raw rax ray raz rbk rbl rbp rcf rea reb ree reg rej rel rem ren res ret rey rga rgn rgr rgs rgu rhp ria rif ril rim rin rir rit riu rjg rkb rkh rkm rkw rm rma rmb rmc rmd rme rmf rmg rmh rmk rml rmm rmn rmo rmp rmq rmu rmw rmx rn rnd rng rnl rnn rnr rnw ro rob roc rod roe rof rog rol rom roo rop ror rou row rpn rpt rri rro rrt rsw rtc rth rtm rub ruc ruf rug rui ruk ruo rup ruq ruu ruy ruz rw rwa rwk rwl rwm rwo rxd rxw saa sab sac sad sae saf saj sak sao saq sar sas sau sav saw sax say sba sbb sbc sbd sbe sbg sbh sbi sbj sbk sbl sbm sbo sbp sbq sbr sbs sbt sbv sbw sbx sby sbz sc scb sce scf scg sch sci scn sco scs scv scw sda sdc sde sdj sdk sdn sdo sdq sdu sdx se sea seb sec sed see sef seg seh sei sej sek sen seo sep seq ser ses set seu sev sew sey sez sfe sfw sg sgb sgc sgd sge sgi sgm sgp sgs sgu sgz sha shb shc she shg shh shj shk sho shp shq shr shs sht shw shy shz sib sid sie sif sig sih sii sij sik sil sim siq sir sis siu siv siw six sja sjb sje sjg sjl sjm sjr sju sjw sk ska skc skd ske skf skg skh ski skm skn sko skp skq sks skt sku skv skw skx sky skz sl slc sld slg slh sli slj sll slm sln slp slr slu slw slx sly slz sm sma smb smc smf smg smj smk sml smn smq smr sms smt smw smx smz sn snc sne snf sng sni snj snk snl snm snn sno snp snq snr sns snu snv snw snx sny snz so sob soc sod soe sok sol soo sop soq sor sos sov sow sox soy soz spb spc spd spe spg spi spk spl spm spn spo spp spq spr sps sq sqa sqh sqm squ sra sre srf srg sri srk srl srm srn sro srq srr srs srt sru srv srw sry ss ssb ssc ssd sse ssf ssg ssj ssl ssm ssn sso ssq sst ssu ssv ssx ssy ssz st sta stb ste stf stg sth sti stj stk stl stm stn sto stp stq str stt stw su sua sub suc sue sug sui suj suk suo suq sur sus sut suv suw suy sv svb svc sve svm svs sw swf swg swj swk swm swo swp swq swr sws swt swu sww swx swy sxb sxe sxn sxr sxs sxu sxw sya syb syi syk sym syo sys syx sza szb szc szg szl szn szp szv szw szy taa tac tad tae taf tag tak tal tan tao tap taq tar tas tau tav taw tax tay taz tba tbc tbd tbe tbf tbg tbh tbi tbj tbl tbm tbn tbo tbp tbs tbt tbu tbv tbw tbx tby tbz tca tcb tcc tcd tce tcf tcg tch tci tck tcm tcp tcq tcs tcu tcw tcz tdc tde tdi tdj tdk tdl tdm tdn tdo tdq tdr tds tdt tdv tdx tdy tea teb tec ted tee teg teh tei tek tem ten teo tep teq ter tet teu tev tew tex tey tfi tfn tfo tfr tft tga tgb tgc tgd tgh tgi tgj tgn tgo tgp tgq tgs tgt tgu tgv tgw tgx tgy tgz thd thh thk thp tht thu thv thy thz tic tif tih tii tik til tim tio tip tiq tis tit tiu tiv tiw tix tiy tja tjg tji tjj tjn tjp tjs tju tjw tk tka tkd tke tkf tkg tkl tkp tkq tkr tku tkv tkw tkx tkz tl tla tlb tlc tld tlf tlg tli tlj tlk tll tlm tln tlp tlq tlr tls tlt tlu tlv tlx tly tma tmb tmc tmd tme tmf tmg tmh tmi tmj tml tmm tmn tmo tmq tmt tmu tmv tmw tmy tmz tn tna tnb tnc tnd tng tnh tni tnk tnl tnm tnn tno tnp tnq tnr tns tnt tnw tnx tny to tob toc tod tof tog toh toi toj tok tol tom too top toq tor tos tou tow tox toy toz tpa tpc tpe tpf tpg tpi tpj tpk tpl tpm tpn tpp tpr tpt tpv tpx tpy tpz tqb tql tqm tqn tqo tqp tqt tqu tqw tr trb trc tre trf trh tri trj trl trn tro trp trq trr trs trt tru trv trx try trz ts tsa tsb tsc tsg tsh tsi tsl tsp tsr tst tsu tsv tsw tsx tsz ttb ttc ttd tte ttf tti ttj ttk ttl ttm ttn ttp ttr ttt ttu ttv ttw tty tua tub tuc tud tue tuf tug tuh tui tuj tul tum tun tuo tuq tus tuu tuv tux tuy tuz tva tvd tve tvi tvk tvl tvm tvo tvs tvt tvu tvw tvx twa twb twd twe twf twg twh twl twn two twp twq twr twt twu tww twx twy txa txe txi txj txm txn txq txs txt txu txx txy ty tya tye tyh tyi tyj tyl tyn typ tys tyt tyu tyx tyy tyz tzh tzj tzl tzm tzn tzo tzx uam uar uba ubi ubl ubr ubu uda udj udl udu ues ufi ugb uge uha uhn uis uiv uji uka ukg ukh ukk ukp ukq uku ukv ukw uky ula ulb ule ulf uli ulk ulm uln ulu ulw uma umb umd umg umi umm umn umo ump umr ums una une ung uni unk unm unn unu unz uon upi upv ura urb urc ure urf urg urh uri urm urn uro urp urr urt uru urv urw urx ury urz usa usi usk usp uss usu uta ute uth utp utr utu uur uve uvh uvl uwa uya uz vae vag vaj val vam van vao vap var vau vbb vbk ve vec vem veo vep ver vi vic vid vif vig vil vin vit viv vka vkj vkk vkl vkm vkn vko vkp vkt vku vkz vlp vls vma vmb vmc vme vmf vmg vmi vmj vmk vml vmm vmp vmq vmr vms vmu vmw vmx vmy vmz vnk vnm vnp vo vor vot vra vro vrs vrt vto vum vun vut vwa wa waa wab wac wad wae waf wag wah wai waj wam wan wap waq war was wat wau wav waw wax way waz wba wbb wbe wbf wbh wbi wbj wbl wbm wbp wbt wbv wbw wca wci wdd wdg wdj wdk wdt wdu wdy wec wed weg weh wei wem weo wep wer wes wet weu wew wfg wga wgb wgg wgi wgo wgu wgy wha whg whk whu wib wic wie wif wig wih wii wij wik wil wim win wir wiu wiv wiy wja wji wka wkd wkr wkw wky wla wlg wlh wli wlm wlr wls wlu wlv wlw wlx wma wmb wmc wmd wmh wmi wmm wmn wmo wms wmt wmw wmx wnb wnc wnd wng wnk wnm wnn wno wnp wnu wnw wny wo woa wob woc wod woe wof wog woi wok wom won woo wor wos wow wpc wrb wrg wrh wri wrk wrl wrm wro wrp wrr wrs wru wrv wrw wrx wrz wsa wsi wsk wsr wss wsu wtf wth wti wtk wtw wua wub wud wul wum wun wur wut wuv wux wuy wwa wwb wwo wwr www wxw wyb wyi wym wyn wyr wyy xaa xab xai xaj xak xam xao xar xat xau xav xaw xay xbb xbd xbe xbg xbi xbj xbm xbn xbp xbr xbw xby xch xda xdk xdo xdy xed xeg xem xer xes xet xeu xgb xgd xgg xgi xgm xgu xgw xh xhv xii xin xir xiy xjb xjt xkb xkd xke xkg xkl xkn xkq xkr xks xkt xku xkv xkw xkx xky xkz xla xma xmb xmc xmd xmg xmh xmj xmm xmo xmp xmq xmt xmu xmv xmw xmx xmy xmz xnb xni xnj xnk xnm xnn xnq xnt xnu xny xnz xoc xod xog xoi xok xom xon xoo xop xor xow xpa xpb xpd xpf xph xpj xpk xpl xpn xpo xpq xpt xpv xpw xpx xpz xra xrb xrd xre xrg xri xrr xru xrw xsb xse xsh xsi xsm xsn xsp xsq xsu xsy xta xtb xtc xtd xte xth xti xtj xtl xtm xtn xtp xts xtt xtu xtv xtw xty xud xul xum xun xuo xut xuu xvn xvo xvs xwa xwd xwe xwj xwk xwl xwr xwt xww xxb xxk xxm xxr xxt xya xyb xyj xyk xyl xyt xyy xzp yaa yab yac yad yae yaf yag yaj yak yal yam yan yao yap yaq yar yas yat yau yav yaw yax yay yaz yba ybb ybe ybj ybl ybm ybn ybo ybx yby ycl ycn yda yde ydk yec yee yei yel yer yes yet yev yey yga ygi ygl ygm ygr ygu ygw yia yii yij yil yim yir yis yka yki ykk ykm yko ykr yky yla ylb yle ylg yli yll ylr ylu yly ymb yme ymg ymk yml ymm ymn ymo ymp ynd yng ynl ynq yns ynu yo yob yog yok yol yom yon yot yra yrb yre yrl yrm yro yrs yrw yry yss ytw yty yua yub yuc yuf yui yuj yul yum yun yup yuq yur yut yuw yuz yva yvt ywa ywg ywn ywr yww yxa yxg yxl yxm yxu yxy yyr yyu za zaa zab zac zad zae zaf zag zah zaj zak zam zao zap zaq zar zas zat zav zaw zax zay zaz zbc zbe zbt zbu zbw zca zea zeg zga zgr zhi zhn zhw zia zik zil zim zin ziw ziz zka zkd zkp zku zla zlm zma zmb zmc zmd zme zmf zmg zmh zmi zmj zmk zml zmm zmn zmo zmp zmq zmr zms zmt zmu zmv zmw zmx zmy zmz zna zne zng znk zns zoc zoh zom zoo zoq zor zos zpa zpb zpc zpd zpe zpf zpg zph zpi zpj zpk zpl zpm zpn zpo zpp zpq zpr zps zpt zpu zpv zpw zpx zpy zpz zrn zro zrs zsa zsr zsu zte ztg ztl ztm ztn ztp ztq zts ztt ztu ztx zty zu zuh zun zuy zyj zyp zza ");
            likelyScriptMap.put("Cans", " cr crj crk crl crm csw iu nsk oj ojs ");
            likelyScriptMap.put("Tfng", " shi tda zen zgh ");
            likelyScriptMap.put("Gong", " wsg ");
            likelyScriptMap.put("Tale", " tdd thi ");
            likelyScriptMap.put("Modi", " omr ");
            likelyScriptMap.put("Hluw", " hlu ");
            likelyScriptMap.put("Kawi", " kaw ");
            likelyScriptMap.put("Gonm", " esg ");
            likelyScriptMap.put("Xpeo", " peo ");
            likelyScriptMap.put("Telu", " cde gau gon key kfc lmn mju nit ort pcj te wbq yeu ");
            likelyScriptMap.put("Guru", " pa ");
            likelyScriptMap.put("Lyci", " imy xlc ");
            likelyScriptMap.put("Talu", " khb ");
            likelyScriptMap.put("Gujr", " cdi dhn dub gas gbl gu ");
            likelyScriptMap.put("Hebr", " aij czk hbo he huy itk iw jbe ji jpa jpr jrb jye lad lsd trg yhd yi yih yud zrp ");
            likelyScriptMap.put("Vaii", " vai ");
            likelyScriptMap.put("Olck", " sat ");
            likelyScriptMap.put("Tagb", " tbk ");
            likelyScriptMap.put("Bopo", " hmj hmq ");
            likelyScriptMap.put("Ogam", " pgl sga xpi ");
            likelyScriptMap.put("Medf", " dmf ");
            likelyScriptMap.put("Sgnw", " ase ");
            likelyScriptMap.put("Cari", " xcr ");
            likelyScriptMap.put("Knda", " kfa kfd kn tcy vmd ");
            likelyScriptMap.put("Arab", " aao abh abv acm acq acw acx adf aeb aec aee aeq afb aib aiq apc apd ar arq ars ary arz ask atn auj auz avd avl ayh ayl ayn ayp azb bal bdz bej bft bgn bgp bhe bhm bjm bqi brh brk bsh bsk chg cja ckb clh dcc def deh dmk dml esh fa fay faz fia fub gbz ggg gha ghr gig gjk gju glh glk gwc gwf gwt gzi hac haz hkh hnd hno hoh hrz hss isk jad jat jbn jdg jnd jog kbu kby kfm khw klj kmz ks ktl kvx kxp lah lki lrc lrk lrl lsa lss luv luz mby mde mfa mfi mhj mki mnj mvy mzn nli nlm ntz nyq odk oru ota pbt phl phr phv plk prc prd prx ps psh psi pst qxq rdb rmt sbn scl sd sdb sdf sdg sdh sds sgr sgy shd shm shu shv siy siz skr smy sqo sqt srh srz ssh sts swb tjo tks tov tra trm trw ug ur ush uzs vaf vgr vmh wbk wlo wne wni wsv xhe xka xkc xkj xkp xvi ydg zba zdj zum ");
            likelyScriptMap.put("Syrc", " amw bhn bjf hrt kqd lhs oar syc syn syr tmr ");
            likelyScriptMap.put("Laoo", " alk brv kjg kuf lbo lo ncq ngt pho sct sqq sss tth tto ");
            likelyScriptMap.put("Lina", " lab ");
            likelyScriptMap.put("Tavt", " blt soa tyr ");
            likelyScriptMap.put("Ethi", " agj ahg alw am anu awn bcq bst byn dox drs gez gmv gof gru har hdy ior kqy ktb mdx mdy muz mvz mym sgw stv ti tig wal xan ");
            likelyScriptMap.put("Tang", " txg ");
            likelyScriptMap.put("Deva", " agi ahr anp anr aph awa bap bee bfb bfy bfz bgc bgd bgq bgw bha bhb bhd bhi bhj bho bhu biy bjj bmj bns bpx bra brd brx btv byh byw cdh cdj cdm chx cih ctn daq dhi dho dhw doi drq dry dty duh dus dwz emg emu fmu gbk gbm gdx ghe gok gom gra gvr gyo hi hlb hne hoc hoj hoy hut jee jml jnl jns jul kex kfb kfk kfp kfq kfr kfs kfx kfy kgj kgy khn kif kip kjl kjo kkt kle klr kmj kok kra kru ksz kyv kyw lae lbf lbm lbr lhm lif lmh loy luu mag mai mgp mjl mjt mjz mkb mke mr mrd mrr mtr mut mwr nao ncd ne new nlx nmm noe noi nwx ola oon pch pci pgg phd phw pum pwr raa rab raf raj rav rji rjs rtw rwr sa sck scp sgj sjp skj soi srx suz swv syw taj tdb tdg tdh tge the thf thl thq thr ths tij tkb tkt twm vah vas vav vay wbr wme wtm xnr xsr ybh ybi ");
            likelyScriptMap.put("Lana", " cuu kkh nod ");
            likelyScriptMap.put("Elym", " xly ");
            likelyScriptMap.put("Sarb", " xsa ");
            likelyScriptMap.put("Batk", " btd btm ");
            likelyScriptMap.put("Rohg", " rhg ");
            likelyScriptMap.put("Jpan", " ams ja yoi ");
            likelyScriptMap.put("Linb", " gmy ");
            likelyScriptMap.put("Prti", " xpr ");
            likelyScriptMap.put("Mlym", " aaf all kfh mjq mjr mjv ml pcf pcg pkr udg yea ");
            likelyScriptMap.put("Rjng", " abl ");
        }
    }

    @Override
    public LocaleProviderAdapter.Type getType() {
        return LocaleProviderAdapter.Type.CLDR;
    }

    @Override
    public String availableLanguageTags(String category) {
        return " en en-Latn-US en-US en-US-POSIX";
    }

    @Override
    public Map<String, String> getLanguageAliasMap() {
        return languageAliasMap;
    }

    @Override
    public Map<String, String> tzCanonicalIDs() {
        return CLDRMapHolder.tzCanonicalIDMap;
    }

    public Map<Locale, String[]> parentLocales() {
        return parentLocalesMap;
    }

    // package access from CLDRLocaleProviderAdapter
    Map<String, String> likelyScriptMap() {
        return CLDRMapHolder.likelyScriptMap;
    }
}
