/*
 * Copyright (c) 2023, Oracle and/or its affiliates. All rights reserved.
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

package jdk.internal.util.regex;

/**
 * Helper class for supporting the GB9c rule in Unicode Text Segmentation TR29
 *
 * <blockquote>
 * GB9c Do not break within certain combinations with Indic_Conjunct_Break (InCB)=Linker.
 *
 * \p{InCB=Consonant} [ \p{InCB=Extend} \p{InCB=Linker} ]* \p{InCB=Linker} [ \p{InCB=Extend} \p{InCB=Linker} ]* x \p{InCB=Consonant}*
 * </blockquote>
 *
 * Code point conditions included in this class are derived from the "Derived Property: Indic_Conjunct_Break"
 * section in DerivedCoreProperties.txt of the Unicode Character Database.
 */
final class IndicConjunctBreak {
    static boolean isLinker(int cp) {
        return
            cp == 0x094D ||
            cp == 0x09CD ||
            cp == 0x0ACD ||
            cp == 0x0B4D ||
            cp == 0x0C4D ||
            cp == 0x0D4D;
    }

    static boolean isExtend(int cp) {
        return
           (cp >= 0x0300 && cp <= 0x034E) ||
           (cp >= 0x0350 && cp <= 0x036F) ||
           (cp >= 0x0483 && cp <= 0x0487) ||
           (cp >= 0x0591 && cp <= 0x05BD) ||
            cp == 0x05BF ||
            cp == 0x05C1 ||
            cp == 0x05C2 ||
            cp == 0x05C4 ||
            cp == 0x05C5 ||
            cp == 0x05C7 ||
           (cp >= 0x0610 && cp <= 0x061A) ||
           (cp >= 0x064B && cp <= 0x065F) ||
            cp == 0x0670 ||
           (cp >= 0x06D6 && cp <= 0x06DC) ||
           (cp >= 0x06DF && cp <= 0x06E4) ||
            cp == 0x06E7 ||
            cp == 0x06E8 ||
           (cp >= 0x06EA && cp <= 0x06ED) ||
            cp == 0x0711 ||
           (cp >= 0x0730 && cp <= 0x074A) ||
           (cp >= 0x07EB && cp <= 0x07F3) ||
            cp == 0x07FD ||
           (cp >= 0x0816 && cp <= 0x0819) ||
           (cp >= 0x081B && cp <= 0x0823) ||
           (cp >= 0x0825 && cp <= 0x0827) ||
           (cp >= 0x0829 && cp <= 0x082D) ||
           (cp >= 0x0859 && cp <= 0x085B) ||
           (cp >= 0x0898 && cp <= 0x089F) ||
           (cp >= 0x08CA && cp <= 0x08E1) ||
           (cp >= 0x08E3 && cp <= 0x08FF) ||
            cp == 0x093C ||
           (cp >= 0x0951 && cp <= 0x0954) ||
            cp == 0x09BC ||
            cp == 0x09FE ||
            cp == 0x0A3C ||
            cp == 0x0ABC ||
            cp == 0x0B3C ||
            cp == 0x0C3C ||
            cp == 0x0C55 ||
            cp == 0x0C56 ||
            cp == 0x0CBC ||
            cp == 0x0D3B ||
            cp == 0x0D3C ||
           (cp >= 0x0E38 && cp <= 0x0E3A) ||
           (cp >= 0x0E48 && cp <= 0x0E4B) ||
           (cp >= 0x0EB8 && cp <= 0x0EBA) ||
           (cp >= 0x0EC8 && cp <= 0x0ECB) ||
            cp == 0x0F18 ||
            cp == 0x0F19 ||
            cp == 0x0F35 ||
            cp == 0x0F37 ||
            cp == 0x0F39 ||
            cp == 0x0F71 ||
            cp == 0x0F72 ||
            cp == 0x0F74 ||
           (cp >= 0x0F7A && cp <= 0x0F7D) ||
            cp == 0x0F80 ||
           (cp >= 0x0F82 && cp <= 0x0F84) ||
            cp == 0x0F86 ||
            cp == 0x0F87 ||
            cp == 0x0FC6 ||
            cp == 0x1037 ||
            cp == 0x1039 ||
            cp == 0x103A ||
            cp == 0x108D ||
           (cp >= 0x135D && cp <= 0x135F) ||
            cp == 0x1714 ||
            cp == 0x17D2 ||
            cp == 0x17DD ||
            cp == 0x18A9 ||
           (cp >= 0x1939 && cp <= 0x193B) ||
            cp == 0x1A17 ||
            cp == 0x1A18 ||
            cp == 0x1A60 ||
           (cp >= 0x1A75 && cp <= 0x1A7C) ||
            cp == 0x1A7F ||
           (cp >= 0x1AB0 && cp <= 0x1ABD) ||
           (cp >= 0x1ABF && cp <= 0x1ACE) ||
            cp == 0x1B34 ||
           (cp >= 0x1B6B && cp <= 0x1B73) ||
            cp == 0x1BAB ||
            cp == 0x1BE6 ||
            cp == 0x1C37 ||
           (cp >= 0x1CD0 && cp <= 0x1CD2) ||
           (cp >= 0x1CD4 && cp <= 0x1CE0) ||
           (cp >= 0x1CE2 && cp <= 0x1CE8) ||
            cp == 0x1CED ||
            cp == 0x1CF4 ||
            cp == 0x1CF8 ||
            cp == 0x1CF9 ||
           (cp >= 0x1DC0 && cp <= 0x1DFF) ||
            cp == 0x200D ||
           (cp >= 0x20D0 && cp <= 0x20DC) ||
            cp == 0x20E1 ||
           (cp >= 0x20E5 && cp <= 0x20F0) ||
           (cp >= 0x2CEF && cp <= 0x2CF1) ||
            cp == 0x2D7F ||
           (cp >= 0x2DE0 && cp <= 0x2DFF) ||
           (cp >= 0x302A && cp <= 0x302F) ||
            cp == 0x3099 ||
            cp == 0x309A ||
            cp == 0xA66F ||
           (cp >= 0xA674 && cp <= 0xA67D) ||
            cp == 0xA69E ||
            cp == 0xA69F ||
            cp == 0xA6F0 ||
            cp == 0xA6F1 ||
            cp == 0xA82C ||
           (cp >= 0xA8E0 && cp <= 0xA8F1) ||
           (cp >= 0xA92B && cp <= 0xA92D) ||
            cp == 0xA9B3 ||
            cp == 0xAAB0 ||
           (cp >= 0xAAB2 && cp <= 0xAAB4) ||
            cp == 0xAAB7 ||
            cp == 0xAAB8 ||
            cp == 0xAABE ||
            cp == 0xAABF ||
            cp == 0xAAC1 ||
            cp == 0xAAF6 ||
            cp == 0xABED ||
            cp == 0xFB1E ||
           (cp >= 0xFE20 && cp <= 0xFE2F) ||
            cp == 0x101FD ||
            cp == 0x102E0 ||
           (cp >= 0x10376 && cp <= 0x1037A) ||
            cp == 0x10A0D ||
            cp == 0x10A0F ||
           (cp >= 0x10A38 && cp <= 0x10A3A) ||
            cp == 0x10A3F ||
            cp == 0x10AE5 ||
            cp == 0x10AE6 ||
           (cp >= 0x10D24 && cp <= 0x10D27) ||
            cp == 0x10EAB ||
            cp == 0x10EAC ||
           (cp >= 0x10EFD && cp <= 0x10EFF) ||
           (cp >= 0x10F46 && cp <= 0x10F50) ||
           (cp >= 0x10F82 && cp <= 0x10F85) ||
            cp == 0x11070 ||
            cp == 0x1107F ||
            cp == 0x110BA ||
           (cp >= 0x11100 && cp <= 0x11102) ||
            cp == 0x11133 ||
            cp == 0x11134 ||
            cp == 0x11173 ||
            cp == 0x111CA ||
            cp == 0x11236 ||
            cp == 0x112E9 ||
            cp == 0x112EA ||
            cp == 0x1133B ||
            cp == 0x1133C ||
           (cp >= 0x11366 && cp <= 0x1136C) ||
           (cp >= 0x11370 && cp <= 0x11374) ||
            cp == 0x11446 ||
            cp == 0x1145E ||
            cp == 0x114C3 ||
            cp == 0x115C0 ||
            cp == 0x116B7 ||
            cp == 0x1172B ||
            cp == 0x1183A ||
            cp == 0x1193E ||
            cp == 0x11943 ||
            cp == 0x11A34 ||
            cp == 0x11A47 ||
            cp == 0x11A99 ||
            cp == 0x11D42 ||
            cp == 0x11D44 ||
            cp == 0x11D45 ||
            cp == 0x11D97 ||
            cp == 0x11F42 ||
           (cp >= 0x16AF0 && cp <= 0x16AF4) ||
           (cp >= 0x16B30 && cp <= 0x16B36) ||
            cp == 0x1BC9E ||
            cp == 0x1D165 ||
           (cp >= 0x1D167 && cp <= 0x1D169) ||
           (cp >= 0x1D16E && cp <= 0x1D172) ||
           (cp >= 0x1D17B && cp <= 0x1D182) ||
           (cp >= 0x1D185 && cp <= 0x1D18B) ||
           (cp >= 0x1D1AA && cp <= 0x1D1AD) ||
           (cp >= 0x1D242 && cp <= 0x1D244) ||
           (cp >= 0x1E000 && cp <= 0x1E006) ||
           (cp >= 0x1E008 && cp <= 0x1E018) ||
           (cp >= 0x1E01B && cp <= 0x1E021) ||
            cp == 0x1E023 ||
            cp == 0x1E024 ||
           (cp >= 0x1E026 && cp <= 0x1E02A) ||
            cp == 0x1E08F ||
           (cp >= 0x1E130 && cp <= 0x1E136) ||
            cp == 0x1E2AE ||
           (cp >= 0x1E2EC && cp <= 0x1E2EF) ||
           (cp >= 0x1E4EC && cp <= 0x1E4EF) ||
           (cp >= 0x1E8D0 && cp <= 0x1E8D6) ||
           (cp >= 0x1E944 && cp <= 0x1E94A);
    }

    static boolean isConsonant(int cp) {
        // fast check - Devanagari to Malayalam
        if (cp < 0x0900 || cp > 0x0D7F) {
            return false;
        }

        return
           (cp >= 0x0915 && cp <= 0x0939) ||
           (cp >= 0x0958 && cp <= 0x095F) ||
           (cp >= 0x0978 && cp <= 0x097F) ||
           (cp >= 0x0995 && cp <= 0x09A8) ||
           (cp >= 0x09AA && cp <= 0x09B0) ||
            cp == 0x09B2 ||
           (cp >= 0x09B6 && cp <= 0x09B9) ||
            cp == 0x09DC ||
            cp == 0x09DD ||
            cp == 0x09DF ||
            cp == 0x09F0 ||
            cp == 0x09F1 ||
           (cp >= 0x0A95 && cp <= 0x0AA8) ||
           (cp >= 0x0AAA && cp <= 0x0AB0) ||
            cp == 0x0AB2 ||
            cp == 0x0AB3 ||
           (cp >= 0x0AB5 && cp <= 0x0AB9) ||
            cp == 0x0AF9 ||
           (cp >= 0x0B15 && cp <= 0x0B28) ||
           (cp >= 0x0B2A && cp <= 0x0B30) ||
            cp == 0x0B32 ||
            cp == 0x0B33 ||
           (cp >= 0x0B35 && cp <= 0x0B39) ||
            cp == 0x0B5C ||
            cp == 0x0B5D ||
            cp == 0x0B5F ||
            cp == 0x0B71 ||
           (cp >= 0x0C15 && cp <= 0x0C28) ||
           (cp >= 0x0C2A && cp <= 0x0C39) ||
           (cp >= 0x0C58 && cp <= 0x0C5A) ||
           (cp >= 0x0D15 && cp <= 0x0D3A);
    }
}
