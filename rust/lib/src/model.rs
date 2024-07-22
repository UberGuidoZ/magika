// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// DO NOT EDIT, see link below for more information:
// https://github.com/google/magika/tree/main/rust/gen

use std::borrow::Cow;

use crate::config::ModelConfig;
use crate::ContentType;

pub(crate) const CONFIG: ModelConfig = ModelConfig {
    beg_size: 2048,
    mid_size: 0,
    end_size: 2048,
    use_inputs_at_offsets: false,
    min_file_size_for_dl: 8,
    padding_token: 256,
    block_size: 4096,
    thresholds: Cow::Borrowed(&THRESHOLDS),
    overwrite_map: Cow::Borrowed(&OVERWRITE_MAP),
};

#[rustfmt::skip]
const THRESHOLDS: [f32; ContentType::SIZE] = [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5];
const OVERWRITE_MAP: [ContentType; ContentType::SIZE] = [
    ContentType::_3gp,
    ContentType::Ace,
    ContentType::Ai,
    ContentType::Aidl,
    ContentType::Apk,
    ContentType::Applebplist,
    ContentType::Appleplist,
    ContentType::Asm,
    ContentType::Asp,
    ContentType::Autohotkey,
    ContentType::Autoit,
    ContentType::Awk,
    ContentType::Batch,
    ContentType::Bazel,
    ContentType::Bib,
    ContentType::Bmp,
    ContentType::Bzip,
    ContentType::C,
    ContentType::Cab,
    ContentType::Cat,
    ContentType::Chm,
    ContentType::Clojure,
    ContentType::Cmake,
    ContentType::Cobol,
    ContentType::Coff,
    ContentType::Coffeescript,
    ContentType::Cpp,
    ContentType::Crt,
    ContentType::Crx,
    ContentType::Cs,
    ContentType::Csproj,
    ContentType::Css,
    ContentType::Csv,
    ContentType::Dart,
    ContentType::Deb,
    ContentType::Dex,
    ContentType::Dicom,
    ContentType::Diff,
    ContentType::Dm,
    ContentType::Dmg,
    ContentType::Doc,
    ContentType::Dockerfile,
    ContentType::Docx,
    ContentType::Dsstore,
    ContentType::Dwg,
    ContentType::Dxf,
    ContentType::Elf,
    ContentType::Elixir,
    ContentType::Emf,
    ContentType::Eml,
    ContentType::Empty,
    ContentType::Epub,
    ContentType::Erb,
    ContentType::Erlang,
    ContentType::Flac,
    ContentType::Flv,
    ContentType::Fortran,
    ContentType::Gemfile,
    ContentType::Gemspec,
    ContentType::Gif,
    ContentType::Gitattributes,
    ContentType::Gitmodules,
    ContentType::Go,
    ContentType::Gradle,
    ContentType::Groovy,
    ContentType::Gzip,
    ContentType::H5,
    ContentType::Handlebars,
    ContentType::Haskell,
    ContentType::Hcl,
    ContentType::Hlp,
    ContentType::Htaccess,
    ContentType::Html,
    ContentType::Icns,
    ContentType::Ico,
    ContentType::Ics,
    ContentType::Ignorefile,
    ContentType::Ini,
    ContentType::Internetshortcut,
    ContentType::Ipynb,
    ContentType::Iso,
    ContentType::Jar,
    ContentType::Java,
    ContentType::Javabytecode,
    ContentType::Javascript,
    ContentType::Jinja,
    ContentType::Jp2,
    ContentType::Jpeg,
    ContentType::Json,
    ContentType::Jsonl,
    ContentType::Julia,
    ContentType::Kotlin,
    ContentType::Latex,
    ContentType::Lha,
    ContentType::Lisp,
    ContentType::Lnk,
    ContentType::Lua,
    ContentType::M3u,
    ContentType::M4,
    ContentType::Macho,
    ContentType::Makefile,
    ContentType::Markdown,
    ContentType::Matlab,
    ContentType::Mht,
    ContentType::Midi,
    ContentType::Mkv,
    ContentType::Mp3,
    ContentType::Mp4,
    ContentType::Mscompress,
    ContentType::Msi,
    ContentType::Mum,
    ContentType::Npy,
    ContentType::Npz,
    ContentType::Nupkg,
    ContentType::Objectivec,
    ContentType::Ocaml,
    ContentType::Odp,
    ContentType::Ods,
    ContentType::Odt,
    ContentType::Ogg,
    ContentType::One,
    ContentType::Onnx,
    ContentType::Otf,
    ContentType::Outlook,
    ContentType::Parquet,
    ContentType::Pascal,
    ContentType::Pcap,
    ContentType::Pdb,
    ContentType::Pdf,
    ContentType::Pebin,
    ContentType::Pem,
    ContentType::Perl,
    ContentType::Php,
    ContentType::Pickle,
    ContentType::Png,
    ContentType::Po,
    ContentType::Postscript,
    ContentType::Powershell,
    ContentType::Ppt,
    ContentType::Pptx,
    ContentType::Prolog,
    ContentType::Proteindb,
    ContentType::Proto,
    ContentType::Psd,
    ContentType::Python,
    ContentType::Pythonbytecode,
    ContentType::Qt,
    ContentType::R,
    ContentType::Rar,
    ContentType::Rdf,
    ContentType::Rpm,
    ContentType::Rst,
    ContentType::Rtf,
    ContentType::Ruby,
    ContentType::Rust,
    ContentType::Scala,
    ContentType::Scss,
    ContentType::Sevenzip,
    ContentType::Sgml,
    ContentType::Shell,
    ContentType::Smali,
    ContentType::Snap,
    ContentType::Solidity,
    ContentType::Sql,
    ContentType::Sqlite,
    ContentType::Squashfs,
    ContentType::Srt,
    ContentType::Stlbinary,
    ContentType::Stltext,
    ContentType::Sum,
    ContentType::Svg,
    ContentType::Swf,
    ContentType::Swift,
    ContentType::Tar,
    ContentType::Tcl,
    ContentType::Textproto,
    ContentType::Tga,
    ContentType::Thumbsdb,
    ContentType::Tiff,
    ContentType::Toml,
    ContentType::Torrent,
    ContentType::Tsv,
    ContentType::Ttf,
    ContentType::Twig,
    ContentType::Txt,
    ContentType::Typescript,
    ContentType::Undefined,
    ContentType::Unknown,
    ContentType::Vba,
    ContentType::Vcxproj,
    ContentType::Verilog,
    ContentType::Vhdl,
    ContentType::Vtt,
    ContentType::Vue,
    ContentType::Wasm,
    ContentType::Wav,
    ContentType::Webm,
    ContentType::Webp,
    ContentType::Winregistry,
    ContentType::Wmf,
    ContentType::Woff,
    ContentType::Woff2,
    ContentType::Xar,
    ContentType::Xls,
    ContentType::Xlsb,
    ContentType::Xlsx,
    ContentType::Xml,
    ContentType::Xpi,
    ContentType::Xz,
    ContentType::Yaml,
    ContentType::Yara,
    ContentType::Zig,
    ContentType::Zip,
    ContentType::Zlibstream,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[allow(dead_code)] // only constructed through transmute
pub(crate) enum Label {
    _3gp,
    Ace,
    Ai,
    Aidl,
    Apk,
    Applebplist,
    Appleplist,
    Asm,
    Asp,
    Autohotkey,
    Autoit,
    Awk,
    Batch,
    Bazel,
    Bib,
    Bmp,
    Bzip,
    C,
    Cab,
    Cat,
    Chm,
    Clojure,
    Cmake,
    Cobol,
    Coff,
    Coffeescript,
    Cpp,
    Crt,
    Crx,
    Cs,
    Csproj,
    Css,
    Csv,
    Dart,
    Deb,
    Dex,
    Dicom,
    Diff,
    Dm,
    Dmg,
    Doc,
    Dockerfile,
    Docx,
    Dsstore,
    Dwg,
    Dxf,
    Elf,
    Elixir,
    Emf,
    Eml,
    Epub,
    Erb,
    Erlang,
    Flac,
    Flv,
    Fortran,
    Gemfile,
    Gemspec,
    Gif,
    Gitattributes,
    Gitmodules,
    Go,
    Gradle,
    Groovy,
    Gzip,
    H5,
    Handlebars,
    Haskell,
    Hcl,
    Hlp,
    Htaccess,
    Html,
    Icns,
    Ico,
    Ics,
    Ignorefile,
    Ini,
    Internetshortcut,
    Ipynb,
    Iso,
    Jar,
    Java,
    Javabytecode,
    Javascript,
    Jinja,
    Jp2,
    Jpeg,
    Json,
    Jsonl,
    Julia,
    Kotlin,
    Latex,
    Lha,
    Lisp,
    Lnk,
    Lua,
    M3u,
    M4,
    Macho,
    Makefile,
    Markdown,
    Matlab,
    Mht,
    Midi,
    Mkv,
    Mp3,
    Mp4,
    Mscompress,
    Msi,
    Mum,
    Npy,
    Npz,
    Nupkg,
    Objectivec,
    Ocaml,
    Odp,
    Ods,
    Odt,
    Ogg,
    One,
    Onnx,
    Otf,
    Outlook,
    Parquet,
    Pascal,
    Pcap,
    Pdb,
    Pdf,
    Pebin,
    Pem,
    Perl,
    Php,
    Pickle,
    Png,
    Po,
    Postscript,
    Powershell,
    Ppt,
    Pptx,
    Prolog,
    Proteindb,
    Proto,
    Psd,
    Python,
    Pythonbytecode,
    Qt,
    R,
    Rar,
    Rdf,
    Rpm,
    Rst,
    Rtf,
    Ruby,
    Rust,
    Scala,
    Scss,
    Sevenzip,
    Sgml,
    Shell,
    Smali,
    Snap,
    Solidity,
    Sql,
    Sqlite,
    Squashfs,
    Srt,
    Stlbinary,
    Stltext,
    Sum,
    Svg,
    Swf,
    Swift,
    Tar,
    Tcl,
    Textproto,
    Tga,
    Thumbsdb,
    Tiff,
    Toml,
    Torrent,
    Tsv,
    Ttf,
    Twig,
    Txt,
    Typescript,
    Unknown,
    Vba,
    Vcxproj,
    Verilog,
    Vhdl,
    Vtt,
    Vue,
    Wasm,
    Wav,
    Webm,
    Webp,
    Winregistry,
    Wmf,
    Woff,
    Woff2,
    Xar,
    Xls,
    Xlsb,
    Xlsx,
    Xml,
    Xpi,
    Xz,
    Yaml,
    Yara,
    Zig,
    Zip,
    Zlibstream,
}

pub(crate) const NUM_LABELS: usize = 212;
impl Label {
    pub(crate) fn content_type(self) -> ContentType {
        match self {
            Label::_3gp => ContentType::_3gp,
            Label::Ace => ContentType::Ace,
            Label::Ai => ContentType::Ai,
            Label::Aidl => ContentType::Aidl,
            Label::Apk => ContentType::Apk,
            Label::Applebplist => ContentType::Applebplist,
            Label::Appleplist => ContentType::Appleplist,
            Label::Asm => ContentType::Asm,
            Label::Asp => ContentType::Asp,
            Label::Autohotkey => ContentType::Autohotkey,
            Label::Autoit => ContentType::Autoit,
            Label::Awk => ContentType::Awk,
            Label::Batch => ContentType::Batch,
            Label::Bazel => ContentType::Bazel,
            Label::Bib => ContentType::Bib,
            Label::Bmp => ContentType::Bmp,
            Label::Bzip => ContentType::Bzip,
            Label::C => ContentType::C,
            Label::Cab => ContentType::Cab,
            Label::Cat => ContentType::Cat,
            Label::Chm => ContentType::Chm,
            Label::Clojure => ContentType::Clojure,
            Label::Cmake => ContentType::Cmake,
            Label::Cobol => ContentType::Cobol,
            Label::Coff => ContentType::Coff,
            Label::Coffeescript => ContentType::Coffeescript,
            Label::Cpp => ContentType::Cpp,
            Label::Crt => ContentType::Crt,
            Label::Crx => ContentType::Crx,
            Label::Cs => ContentType::Cs,
            Label::Csproj => ContentType::Csproj,
            Label::Css => ContentType::Css,
            Label::Csv => ContentType::Csv,
            Label::Dart => ContentType::Dart,
            Label::Deb => ContentType::Deb,
            Label::Dex => ContentType::Dex,
            Label::Dicom => ContentType::Dicom,
            Label::Diff => ContentType::Diff,
            Label::Dm => ContentType::Dm,
            Label::Dmg => ContentType::Dmg,
            Label::Doc => ContentType::Doc,
            Label::Dockerfile => ContentType::Dockerfile,
            Label::Docx => ContentType::Docx,
            Label::Dsstore => ContentType::Dsstore,
            Label::Dwg => ContentType::Dwg,
            Label::Dxf => ContentType::Dxf,
            Label::Elf => ContentType::Elf,
            Label::Elixir => ContentType::Elixir,
            Label::Emf => ContentType::Emf,
            Label::Eml => ContentType::Eml,
            Label::Epub => ContentType::Epub,
            Label::Erb => ContentType::Erb,
            Label::Erlang => ContentType::Erlang,
            Label::Flac => ContentType::Flac,
            Label::Flv => ContentType::Flv,
            Label::Fortran => ContentType::Fortran,
            Label::Gemfile => ContentType::Gemfile,
            Label::Gemspec => ContentType::Gemspec,
            Label::Gif => ContentType::Gif,
            Label::Gitattributes => ContentType::Gitattributes,
            Label::Gitmodules => ContentType::Gitmodules,
            Label::Go => ContentType::Go,
            Label::Gradle => ContentType::Gradle,
            Label::Groovy => ContentType::Groovy,
            Label::Gzip => ContentType::Gzip,
            Label::H5 => ContentType::H5,
            Label::Handlebars => ContentType::Handlebars,
            Label::Haskell => ContentType::Haskell,
            Label::Hcl => ContentType::Hcl,
            Label::Hlp => ContentType::Hlp,
            Label::Htaccess => ContentType::Htaccess,
            Label::Html => ContentType::Html,
            Label::Icns => ContentType::Icns,
            Label::Ico => ContentType::Ico,
            Label::Ics => ContentType::Ics,
            Label::Ignorefile => ContentType::Ignorefile,
            Label::Ini => ContentType::Ini,
            Label::Internetshortcut => ContentType::Internetshortcut,
            Label::Ipynb => ContentType::Ipynb,
            Label::Iso => ContentType::Iso,
            Label::Jar => ContentType::Jar,
            Label::Java => ContentType::Java,
            Label::Javabytecode => ContentType::Javabytecode,
            Label::Javascript => ContentType::Javascript,
            Label::Jinja => ContentType::Jinja,
            Label::Jp2 => ContentType::Jp2,
            Label::Jpeg => ContentType::Jpeg,
            Label::Json => ContentType::Json,
            Label::Jsonl => ContentType::Jsonl,
            Label::Julia => ContentType::Julia,
            Label::Kotlin => ContentType::Kotlin,
            Label::Latex => ContentType::Latex,
            Label::Lha => ContentType::Lha,
            Label::Lisp => ContentType::Lisp,
            Label::Lnk => ContentType::Lnk,
            Label::Lua => ContentType::Lua,
            Label::M3u => ContentType::M3u,
            Label::M4 => ContentType::M4,
            Label::Macho => ContentType::Macho,
            Label::Makefile => ContentType::Makefile,
            Label::Markdown => ContentType::Markdown,
            Label::Matlab => ContentType::Matlab,
            Label::Mht => ContentType::Mht,
            Label::Midi => ContentType::Midi,
            Label::Mkv => ContentType::Mkv,
            Label::Mp3 => ContentType::Mp3,
            Label::Mp4 => ContentType::Mp4,
            Label::Mscompress => ContentType::Mscompress,
            Label::Msi => ContentType::Msi,
            Label::Mum => ContentType::Mum,
            Label::Npy => ContentType::Npy,
            Label::Npz => ContentType::Npz,
            Label::Nupkg => ContentType::Nupkg,
            Label::Objectivec => ContentType::Objectivec,
            Label::Ocaml => ContentType::Ocaml,
            Label::Odp => ContentType::Odp,
            Label::Ods => ContentType::Ods,
            Label::Odt => ContentType::Odt,
            Label::Ogg => ContentType::Ogg,
            Label::One => ContentType::One,
            Label::Onnx => ContentType::Onnx,
            Label::Otf => ContentType::Otf,
            Label::Outlook => ContentType::Outlook,
            Label::Parquet => ContentType::Parquet,
            Label::Pascal => ContentType::Pascal,
            Label::Pcap => ContentType::Pcap,
            Label::Pdb => ContentType::Pdb,
            Label::Pdf => ContentType::Pdf,
            Label::Pebin => ContentType::Pebin,
            Label::Pem => ContentType::Pem,
            Label::Perl => ContentType::Perl,
            Label::Php => ContentType::Php,
            Label::Pickle => ContentType::Pickle,
            Label::Png => ContentType::Png,
            Label::Po => ContentType::Po,
            Label::Postscript => ContentType::Postscript,
            Label::Powershell => ContentType::Powershell,
            Label::Ppt => ContentType::Ppt,
            Label::Pptx => ContentType::Pptx,
            Label::Prolog => ContentType::Prolog,
            Label::Proteindb => ContentType::Proteindb,
            Label::Proto => ContentType::Proto,
            Label::Psd => ContentType::Psd,
            Label::Python => ContentType::Python,
            Label::Pythonbytecode => ContentType::Pythonbytecode,
            Label::Qt => ContentType::Qt,
            Label::R => ContentType::R,
            Label::Rar => ContentType::Rar,
            Label::Rdf => ContentType::Rdf,
            Label::Rpm => ContentType::Rpm,
            Label::Rst => ContentType::Rst,
            Label::Rtf => ContentType::Rtf,
            Label::Ruby => ContentType::Ruby,
            Label::Rust => ContentType::Rust,
            Label::Scala => ContentType::Scala,
            Label::Scss => ContentType::Scss,
            Label::Sevenzip => ContentType::Sevenzip,
            Label::Sgml => ContentType::Sgml,
            Label::Shell => ContentType::Shell,
            Label::Smali => ContentType::Smali,
            Label::Snap => ContentType::Snap,
            Label::Solidity => ContentType::Solidity,
            Label::Sql => ContentType::Sql,
            Label::Sqlite => ContentType::Sqlite,
            Label::Squashfs => ContentType::Squashfs,
            Label::Srt => ContentType::Srt,
            Label::Stlbinary => ContentType::Stlbinary,
            Label::Stltext => ContentType::Stltext,
            Label::Sum => ContentType::Sum,
            Label::Svg => ContentType::Svg,
            Label::Swf => ContentType::Swf,
            Label::Swift => ContentType::Swift,
            Label::Tar => ContentType::Tar,
            Label::Tcl => ContentType::Tcl,
            Label::Textproto => ContentType::Textproto,
            Label::Tga => ContentType::Tga,
            Label::Thumbsdb => ContentType::Thumbsdb,
            Label::Tiff => ContentType::Tiff,
            Label::Toml => ContentType::Toml,
            Label::Torrent => ContentType::Torrent,
            Label::Tsv => ContentType::Tsv,
            Label::Ttf => ContentType::Ttf,
            Label::Twig => ContentType::Twig,
            Label::Txt => ContentType::Txt,
            Label::Typescript => ContentType::Typescript,
            Label::Unknown => ContentType::Unknown,
            Label::Vba => ContentType::Vba,
            Label::Vcxproj => ContentType::Vcxproj,
            Label::Verilog => ContentType::Verilog,
            Label::Vhdl => ContentType::Vhdl,
            Label::Vtt => ContentType::Vtt,
            Label::Vue => ContentType::Vue,
            Label::Wasm => ContentType::Wasm,
            Label::Wav => ContentType::Wav,
            Label::Webm => ContentType::Webm,
            Label::Webp => ContentType::Webp,
            Label::Winregistry => ContentType::Winregistry,
            Label::Wmf => ContentType::Wmf,
            Label::Woff => ContentType::Woff,
            Label::Woff2 => ContentType::Woff2,
            Label::Xar => ContentType::Xar,
            Label::Xls => ContentType::Xls,
            Label::Xlsb => ContentType::Xlsb,
            Label::Xlsx => ContentType::Xlsx,
            Label::Xml => ContentType::Xml,
            Label::Xpi => ContentType::Xpi,
            Label::Xz => ContentType::Xz,
            Label::Yaml => ContentType::Yaml,
            Label::Yara => ContentType::Yara,
            Label::Zig => ContentType::Zig,
            Label::Zip => ContentType::Zip,
            Label::Zlibstream => ContentType::Zlibstream,
        }
    }
}
