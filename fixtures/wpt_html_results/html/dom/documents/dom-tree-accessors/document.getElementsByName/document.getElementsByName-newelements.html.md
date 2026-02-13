# html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-newelements.html

Counts:
- errors: 1
- warnings: 9
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-newelements.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>getElementsByName and newly introduced HTML elements</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-getelementsbyname">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
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
<source name="source">
</div>
<script>
var testDiv = document.getElementById("test");
for (var i = 0; i < testDiv.children.length; i++) {
  var name = testDiv.children[i].getAttribute("name");
  test(function() {
    assert_equals(document.getElementsByName(name).length, 1);
    assert_equals(document.getElementsByName(name)[0],
                  document.getElementsByTagName(name)[0]);
  }, 'getElementsByName("' + name + '")');
}
</script>
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
        "byte_end": 423,
        "byte_start": 413,
        "col": 25,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.article.lacks_heading",
      "message": "Article lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all articles.",
      "severity": "Warning",
      "span": {
        "byte_end": 458,
        "byte_start": 448,
        "col": 25,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 757,
        "byte_start": 749,
        "col": 21,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.meter.missing_value",
      "message": "Element “meter” is missing required attribute “value”.",
      "severity": "Warning",
      "span": {
        "byte_end": 804,
        "byte_start": 784,
        "col": 1,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “command” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 933,
        "byte_start": 909,
        "col": 1,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “command” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 933,
        "byte_start": 909,
        "col": 1,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1004,
        "byte_start": 994,
        "col": 25,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.element.keygen.obsolete",
      "message": "The “keygen” element is obsolete.",
      "severity": "Warning",
      "span": {
        "byte_end": 1065,
        "byte_start": 1043,
        "col": 1,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.ruby.missing.rp_rt",
      "message": "Element “ruby” is missing a required instance of one or more of the following child elements: “rp”, “rt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1125,
        "byte_start": 1107,
        "col": 1,
        "line": 32
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-newelements.html"
}
```
