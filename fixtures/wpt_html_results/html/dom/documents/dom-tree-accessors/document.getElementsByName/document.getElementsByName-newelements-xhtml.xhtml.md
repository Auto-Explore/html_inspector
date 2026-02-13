# html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-newelements-xhtml.xhtml

Counts:
- errors: 0
- warnings: 9
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-newelements-xhtml.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>getElementsByName and newly introduced HTML elements</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com"/>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-getelementsbyname"/>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<div id="log"></div>
<div id="test">
<section name="section"></section>
<article name="article"></article>
<aside name="aside"></aside>
<hgroup name="hgroup"></hgroup>
<header name="header"></header>
<footer name="footer"></footer>
<nav name="nav"></nav>
<dialog name="dialog"></dialog>
<figure name="figure"></figure>
<audio name="audio"></audio>
<video name="video"></video>
<embed name="embed"></embed>
<mark name="mark"></mark>
<meter name="meter"></meter>
<progress name="progress"></progress>
<time name="time"></time>
<canvas name="canvas"></canvas>
<command name="command"></command>
<menu name="menu"></menu>
<details name="details"></details>
<datalist name="datalist"></datalist>
<keygen name="keygen"></keygen>
<output name="output"></output>
<ruby name="ruby"></ruby>
<rt name="rt"></rt>
<rp name="rp"></rp>
<source name="source"/>
</div>
<script>
test(function() {
  assert_equals(document.getElementsByName("section").length, 1);
  assert_equals(document.getElementsByName("section")[0],
                document.getElementsByTagName("section")[0]);
  assert_equals(document.getElementsByName("article").length, 1);
  assert_equals(document.getElementsByName("article")[0],
                document.getElementsByTagName("article")[0]);
  assert_equals(document.getElementsByName("aside").length, 1);
  assert_equals(document.getElementsByName("aside")[0],
                document.getElementsByTagName("aside")[0]);
  assert_equals(document.getElementsByName("hgroup").length, 1);
  assert_equals(document.getElementsByName("hgroup")[0],
                document.getElementsByTagName("hgroup")[0]);
  assert_equals(document.getElementsByName("header").length, 1);
  assert_equals(document.getElementsByName("header")[0],
                document.getElementsByTagName("header")[0]);
  assert_equals(document.getElementsByName("footer").length, 1);
  assert_equals(document.getElementsByName("footer")[0],
                document.getElementsByTagName("footer")[0]);
  assert_equals(document.getElementsByName("nav").length, 1);
  assert_equals(document.getElementsByName("nav")[0],
                document.getElementsByTagName("nav")[0]);
  assert_equals(document.getElementsByName("dialog").length, 1);
  assert_equals(document.getElementsByName("dialog")[0],
                document.getElementsByTagName("dialog")[0]);
  assert_equals(document.getElementsByName("figure").length, 1);
  assert_equals(document.getElementsByName("figure")[0],
                document.getElementsByTagName("figure")[0]);
  assert_equals(document.getElementsByName("audio").length, 1);
  assert_equals(document.getElementsByName("audio")[0],
                document.getElementsByTagName("audio")[0]);
  assert_equals(document.getElementsByName("video").length, 1);
  assert_equals(document.getElementsByName("video")[0],
                document.getElementsByTagName("video")[0]);
  assert_equals(document.getElementsByName("embed").length, 1);
  assert_equals(document.getElementsByName("embed")[0],
                document.getElementsByTagName("embed")[0]);
  assert_equals(document.getElementsByName("mark").length, 1);
  assert_equals(document.getElementsByName("mark")[0],
                document.getElementsByTagName("mark")[0]);
  assert_equals(document.getElementsByName("meter").length, 1);
  assert_equals(document.getElementsByName("meter")[0],
                document.getElementsByTagName("meter")[0]);
  assert_equals(document.getElementsByName("progress").length, 1);
  assert_equals(document.getElementsByName("progress")[0],
                document.getElementsByTagName("progress")[0]);
  assert_equals(document.getElementsByName("time").length, 1);
  assert_equals(document.getElementsByName("time")[0],
                document.getElementsByTagName("time")[0]);
  assert_equals(document.getElementsByName("canvas").length, 1);
  assert_equals(document.getElementsByName("canvas")[0],
                document.getElementsByTagName("canvas")[0]);
  assert_equals(document.getElementsByName("command").length, 1);
  assert_equals(document.getElementsByName("command")[0],
                document.getElementsByTagName("command")[0]);
  assert_equals(document.getElementsByName("menu").length, 1);
  assert_equals(document.getElementsByName("menu")[0],
                document.getElementsByTagName("menu")[0]);
  assert_equals(document.getElementsByName("details").length, 1);
  assert_equals(document.getElementsByName("details")[0],
                document.getElementsByTagName("details")[0]);
  assert_equals(document.getElementsByName("datalist").length, 1);
  assert_equals(document.getElementsByName("datalist")[0],
                document.getElementsByTagName("datalist")[0]);
  assert_equals(document.getElementsByName("keygen").length, 1);
  assert_equals(document.getElementsByName("keygen")[0],
                document.getElementsByTagName("keygen")[0]);
  assert_equals(document.getElementsByName("output").length, 1);
  assert_equals(document.getElementsByName("output")[0],
                document.getElementsByTagName("output")[0]);
  assert_equals(document.getElementsByName("ruby").length, 1);
  assert_equals(document.getElementsByName("ruby")[0],
                document.getElementsByTagName("ruby")[0]);
  assert_equals(document.getElementsByName("rt").length, 1);
  assert_equals(document.getElementsByName("rt")[0],
                document.getElementsByTagName("rt")[0]);
  assert_equals(document.getElementsByName("rp").length, 1);
  assert_equals(document.getElementsByName("rp")[0],
                document.getElementsByTagName("rp")[0]);
  assert_equals(document.getElementsByName("source").length, 1);
  assert_equals(document.getElementsByName("source")[0],
                document.getElementsByTagName("source")[0]);
});
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.section.lacks_heading",
      "message": "Section lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all sections, or else use a “div” element instead for any cases where no heading is needed.",
      "severity": "Warning",
      "span": {
        "byte_end": 475,
        "byte_start": 465,
        "col": 25,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.article.lacks_heading",
      "message": "Article lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all articles.",
      "severity": "Warning",
      "span": {
        "byte_end": 510,
        "byte_start": 500,
        "col": 25,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.meter.missing_value",
      "message": "Element “meter” is missing required attribute “value”.",
      "severity": "Warning",
      "span": {
        "byte_end": 856,
        "byte_start": 836,
        "col": 1,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “command” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 985,
        "byte_start": 961,
        "col": 1,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “command” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 985,
        "byte_start": 961,
        "col": 1,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1056,
        "byte_start": 1046,
        "col": 25,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.element.keygen.obsolete",
      "message": "The “keygen” element is obsolete.",
      "severity": "Warning",
      "span": {
        "byte_end": 1117,
        "byte_start": 1095,
        "col": 1,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.ruby.missing.rp_rt",
      "message": "Element “ruby” is missing a required instance of one or more of the following child elements: “rp”, “rt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1177,
        "byte_start": 1159,
        "col": 1,
        "line": 35
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-newelements-xhtml.xhtml"
}
```
