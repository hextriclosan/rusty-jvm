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

/*
 * COPYRIGHT AND PERMISSION NOTICE
 *
 * Copyright (c) 1991-2022 Unicode, Inc. All rights reserved.
 * Distributed under the Terms of Use in https://www.unicode.org/copyright.html.
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of the Unicode data files and any associated documentation
 * (the "Data Files") or Unicode software and any associated documentation
 * (the "Software") to deal in the Data Files or Software
 * without restriction, including without limitation the rights to use,
 * copy, modify, merge, publish, distribute, and/or sell copies of
 * the Data Files or Software, and to permit persons to whom the Data Files
 * or Software are furnished to do so, provided that either
 * (a) this copyright and permission notice appear with all copies
 * of the Data Files or Software, or
 * (b) this copyright and permission notice appear in associated
 * Documentation.
 *
 * THE DATA FILES AND SOFTWARE ARE PROVIDED "AS IS", WITHOUT WARRANTY OF
 * ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
 * WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT OF THIRD PARTY RIGHTS.
 * IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS INCLUDED IN THIS
 * NOTICE BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL
 * DAMAGES, OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THE DATA FILES OR SOFTWARE.
 *
 * Except as contained in this notice, the name of a copyright holder
 * shall not be used in advertising or otherwise to promote the sale,
 * use or other dealings in these Data Files or Software without prior
 * written authorization of the copyright holder.
 */

package sun.util.resources.cldr;

import sun.util.resources.TimeZoneNamesBundle;

public class TimeZoneNames extends TimeZoneNamesBundle {
    @Override
    protected final Object[][] getContents() {
        final String[] ETC_UTC = new String[] {
               "",
               "UTC",
               "",
               "",
               "",
               "",
            };
        final Object[][] data = new Object[][] {
            { "Africa/Casablanca",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "UTC", ETC_UTC },
            { "CET",
                new String[] {
                    "",
                    "CET",
                    "",
                    "CEST",
                    "",
                    "CET",
                }
            },
            { "EET",
                new String[] {
                    "",
                    "EET",
                    "",
                    "EEST",
                    "",
                    "EET",
                }
            },
            { "IET",
                new String[] {
                    "",
                    "EST",
                    "",
                    "EDT",
                    "",
                    "ET",
                }
            },
            { "MET",
                new String[] {
                    "",
                    "MET",
                    "",
                    "MEST",
                    "",
                    "MET",
                }
            },
            { "UCT",
                new String[] {
                    "",
                    "UTC",
                    "",
                    "UTC",
                    "",
                    "UTC",
                }
            },
            { "WET",
                new String[] {
                    "",
                    "WET",
                    "",
                    "WEST",
                    "",
                    "WET",
                }
            },
            { "Zulu",
                new String[] {
                    "",
                    "UTC",
                    "",
                    "UTC",
                    "",
                    "UTC",
                }
            },
            { "Etc/UCT",
                new String[] {
                    "",
                    "UTC",
                    "",
                    "UTC",
                    "",
                    "UTC",
                }
            },
            { "Etc/UTC", ETC_UTC },
            { "Etc/Zulu",
                new String[] {
                    "",
                    "UTC",
                    "",
                    "UTC",
                    "",
                    "UTC",
                }
            },
            { "Etc/GMT+1",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT+2",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT+3",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT+4",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT+5",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT+6",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT+7",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT+8",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT+9",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT-1",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT-2",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT-3",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT-4",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT-5",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT-6",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT-7",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT-8",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT-9",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Universal",
                new String[] {
                    "",
                    "UTC",
                    "",
                    "UTC",
                    "",
                    "UTC",
                }
            },
            { "Asia/Amman",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Asia/Tomsk",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT+10",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT+11",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT+12",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT-10",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT-11",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT-12",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT-13",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/GMT-14",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Europe/Kyiv",
                new String[] {
                    "",
                    "EET",
                    "",
                    "EEST",
                    "",
                    "EET",
                }
            },
            { "Asia/Barnaul",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Europe/Kirov",
                new String[] {
                    "",
                    "MSK",
                    "",
                    "MSK",
                    "",
                    "MSK",
                }
            },
            { "Asia/Damascus",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Etc/Universal",
                new String[] {
                    "",
                    "UTC",
                    "",
                    "UTC",
                    "",
                    "UTC",
                }
            },
            { "Asia/Famagusta",
                new String[] {
                    "",
                    "EET",
                    "",
                    "EEST",
                    "",
                    "EET",
                }
            },
            { "Europe/Saratov",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Africa/El_Aaiun",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Europe/Astrakhan",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Europe/Ulyanovsk",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Antarctica/Palmer",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Asia/Srednekolymsk",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "America/Punta_Arenas",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "Pacific/Bougainville",
                new String[] {
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                }
            },
            { "timezone.excity.Asia/Macau", "Macao" },
            { "timezone.excity.Asia/Saigon", "Ho Chi Minh" },
            { "timezone.excity.Etc/Unknown", "Unknown" },
            { "timezone.excity.Europe/Kiev", "Kyiv" },
            { "America/Indiana/Indianapolis",
                new String[] {
                    "",
                    "EST",
                    "",
                    "EDT",
                    "",
                    "ET",
                }
            },
            { "timezone.excity.Asia/Rangoon", "Yangon" },
            { "timezone.excity.Pacific/Truk", "Chuuk" },
            { "timezone.excity.Africa/Asmera", "Asmara" },
            { "timezone.excity.Asia/Calcutta", "Kolkata" },
            { "timezone.excity.Asia/Katmandu", "Kathmandu" },
            { "timezone.excity.America/Cancun", "Canc\u00fan" },
            { "timezone.excity.America/Merida", "M\u00e9rida" },
            { "timezone.excity.Indian/Reunion", "R\u00e9union" },
            { "timezone.excity.Pacific/Ponape", "Pohnpei" },
            { "timezone.excity.Africa/Sao_Tome", "S\u00e3o Tom\u00e9" },
            { "timezone.excity.America/Curacao", "Cura\u00e7ao" },
            { "timezone.excity.America/Godthab", "Nuuk" },
            { "timezone.excity.Atlantic/Faeroe", "Faroe" },
            { "timezone.excity.America/Asuncion", "Asunci\u00f3n" },
            { "timezone.excity.America/St_Johns", "St. John\u2019s" },
            { "timezone.excity.America/St_Kitts", "St. Kitts" },
            { "timezone.excity.America/St_Lucia", "St. Lucia" },
            { "timezone.excity.America/St_Thomas", "St. Thomas" },
            { "timezone.excity.America/St_Vincent", "St. Vincent" },
            { "timezone.excity.Atlantic/St_Helena", "St. Helena" },
            { "timezone.excity.America/Indiana/Knox", "Knox, Indiana" },
            { "timezone.excity.America/Scoresbysund", "Ittoqqortoormiit" },
            { "timezone.excity.America/Ciudad_Juarez", "Ciudad Ju\u00e1rez" },
            { "timezone.excity.America/Coral_Harbour", "Atikokan" },
            { "timezone.excity.America/Indiana/Vevay", "Vevay, Indiana" },
            { "timezone.excity.America/Lower_Princes", "Lower Prince\u2019s Quarter" },
            { "timezone.excity.America/St_Barthelemy", "St. Barth\u00e9lemy" },
            { "timezone.excity.America/Bahia_Banderas", "Bah\u00eda de Banderas" },
            { "timezone.excity.America/Indiana/Marengo", "Marengo, Indiana" },
            { "timezone.excity.America/Indiana/Winamac", "Winamac, Indiana" },
            { "timezone.excity.America/Indiana/Tell_City", "Tell City, Indiana" },
            { "timezone.excity.America/Indiana/Vincennes", "Vincennes, Indiana" },
            { "timezone.excity.Antarctica/DumontDUrville", "Dumont d\u2019Urville" },
            { "timezone.excity.America/Indiana/Petersburg", "Petersburg, Indiana" },
            { "timezone.excity.America/Kentucky/Monticello", "Monticello, Kentucky" },
            { "timezone.excity.America/North_Dakota/Beulah", "Beulah, North Dakota" },
            { "timezone.excity.America/North_Dakota/Center", "Center, North Dakota" },
            { "timezone.excity.America/North_Dakota/New_Salem", "New Salem, North Dakota" },
        };
        return data;
    }
}
