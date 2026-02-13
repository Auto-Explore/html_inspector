# html/dom/historical.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/historical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Historical HTML APIs</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<attachment></attachment>
<applet name=war align=left></applet>
<layer></layer>
<nolayer></nolayer>
<script>
test(() => {
  assert_array_equals(document.applets, []);
}, "document.applets is always empty");

[
  "attachment",
  "applet",
  "layer",
  "nolayer"
].forEach(name => {
  test(() => {
    const ap = document.getElementsByTagName(name)[0];
    assert_true(ap instanceof window.HTMLUnknownElement);
  }, `<${name}> is HTMLUnknownElement`);
});

test(() => {
  assert_equals(self.HTMLAppletElement, undefined);
}, "HTMLAppletElement is no more")

test(() => {
  assert_equals(document.all.war, undefined);
}, "document.all cannot find applet")

test(() => {
  assert_equals(document.war, undefined);
}, "document cannot find applet")

test(() => {
  assert_equals(self.war, undefined);
}, "window cannot find applet")

test(() => {
  assert_equals(self.getComputedStyle(document.getElementsByTagName("applet")[0], "").cssFloat, "none");
}, "applet is not styled")

// removed in https://github.com/whatwg/html/commit/e383ae23776362cafb2fb4bbba70c8c9080d4b0f
test(() => {
  assert_false("HTMLTableDataCellElement" in window);
}, "HTMLTableDataCellElement interface is removed")

test(() => {
  assert_false("HTMLTableHeaderCellElement" in window);
}, "HTMLTableHeaderCellElement interface is removed")

// removed in https://github.com/whatwg/html/commit/6e4bcf5630d08e03212ad4e1a3c78beecf2a92fa
test(() => {
  assert_false("initHashChangeEvent" in HashChangeEvent.prototype);
}, "HashChangeEvent's initHashChangeEvent method is removed")
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.applet.obsolete",
      "message": "The “applet” element is obsolete. Use “embed” or “object” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 227,
        "byte_start": 199,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “layer” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 244,
        "byte_start": 237,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “layer” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 244,
        "byte_start": 237,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “nolayer” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 262,
        "byte_start": 253,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “nolayer” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 262,
        "byte_start": 253,
        "col": 1,
        "line": 9
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
  "source_name": "html/dom/historical.html"
}
```
