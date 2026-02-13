# html/browsers/the-window-object/named-access-on-the-window-object/named-objects.html

Counts:
- errors: 5
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/named-access-on-the-window-object/named-objects.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: Named access on the Window object</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/multipage/browsers.html#named-access-on-the-window-object">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div style="display:none">
  <p name="a" id="p1"></p>
  <a name="a" id="a1" href="#"></a>
  <area name="a" id="area1"></area>
  <embed name="a" id="embed1"></embed>
  <form name="a" id="form1"></form>
  <img name="a" id="img1">
  <object name="a" id="obj1"></object>
  <span name="a" id="span1"></span>

  <b id="b" name="c"></b>
  <a name="c"></a>
  <iframe name="c" id="fm1"></iframe>
  <iframe name="c" id="fm2" src="test.html" onload="on_load()"></iframe>
  <input id="b"></input>
  <span id="d"></span>
  <a name=""></a>
  <b id=""></b>
</div>
<script>

test(function() {
  assert_equals(window['c'], document.getElementById("fm1").contentWindow, "The first iframe's window should be returned.");
}, "Check if the first nested browsing context is returned by window['c']");

test(function() {
  assert_true(window['a'] instanceof HTMLCollection);
  assert_array_equals(window['a'],
                      [ document.getElementById('embed1'),
                        document.getElementById('form1'), document.getElementById('img1'),
                        document.getElementById('obj1') ],
                      "The elements are not in tree order.");

  document.getElementById('form1').setAttribute("name", "");
  document.getElementById('embed1').setAttribute("name", "");
  assert_array_equals(window['a'],
                      [ document.getElementById('img1'),
                        document.getElementById('obj1') ],
                      "Window['a'] should not contain the elements with empty name attribute.");
}, "Check if window['a'] contains all embed, form, img, and object elements, and their order");

var t = async_test("Check that window['fs'] does not return the frameset element with name='fs' (historical)");
function on_load () {
  t.step(function () {
    assert_equals(document.getElementById('fm2').contentWindow['fs'],
                  undefined,
                  "The frameset element should not be returned.");
  });
  t.done();
}

test(function() {
  assert_true(window['b'] instanceof HTMLCollection);
  assert_array_equals(window['b'], [document.getElementsByTagName('b')[0], document.getElementsByTagName('input')[0]]);

  document.getElementsByTagName('b')[0].setAttribute("id", "");
  assert_equals(window['b'], document.getElementsByTagName('input')[0],
                "The window['b'] should not contain the elements with empty id attribute.");
}, "Check if window['b'] returns the elements with the id='b'");

test(function() {
  assert_equals(window['d'], document.getElementById('d'));
}, "Check if window['d'] returns the element with id='d'");

test(function() {
  assert_equals(window[''], undefined, "The window[''] should be undefined");
}, "Check widow[''] when there are some elements with empty id or name attribute");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.a.name.obsolete",
      "message": "The “name” attribute on the “a” element is obsolete. Consider putting an “id” attribute on the nearest container instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 496,
        "byte_start": 467,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.area.map_ancestor.missing",
      "message": "The “area” element must have a “map” ancestor.",
      "severity": "Error",
      "span": {
        "byte_end": 529,
        "byte_start": 503,
        "col": 3,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “area”.",
      "severity": "Error",
      "span": {
        "byte_end": 536,
        "byte_start": 529,
        "col": 29,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 575,
        "byte_start": 567,
        "col": 31,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 638,
        "byte_start": 614,
        "col": 3,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 638,
        "byte_start": 614,
        "col": 3,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 668,
        "byte_start": 641,
        "col": 3,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.a.name.obsolete",
      "message": "The “name” attribute on the “a” element is obsolete. Consider putting an “id” attribute on the nearest container instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 755,
        "byte_start": 743,
        "col": 3,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “b”.",
      "severity": "Error",
      "span": {
        "byte_end": 887,
        "byte_start": 873,
        "col": 3,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 895,
        "byte_start": 887,
        "col": 17,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.a.name.obsolete",
      "message": "The “name” attribute on the “a” element is obsolete. Consider putting an “id” attribute on the nearest container instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 932,
        "byte_start": 921,
        "col": 3,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.id.invalid",
      "message": "Bad value “” for attribute “id” on element “b”.",
      "severity": "Warning",
      "span": {
        "byte_end": 948,
        "byte_start": 939,
        "col": 3,
        "line": 26
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
  "source_name": "html/browsers/the-window-object/named-access-on-the-window-object/named-objects.html"
}
```
