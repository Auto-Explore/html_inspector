# html/rendering/widgets/textarea-scrollbar-sizing-001.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/textarea-scrollbar-sizing-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/C/#the-textarea-element-2">
<title>Test textarea width and height accounting for scrollbars</title>
<meta name="author" title="Luke Warlow" href="mailto:lwarlow@igalia.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<textarea id="rows1" rows="1"></textarea>
<textarea id="rows1Hidden" rows="1" style="overflow-x: hidden"></textarea>
<textarea id="rows1Scroll" rows="1" style="overflow-x: scroll"></textarea>
<textarea id="cols3" cols="3"></textarea>
<textarea id="cols3Hidden" cols="3" style="overflow-y: hidden"></textarea>
<textarea id="cols3Scroll" cols="3" style="overflow-y: scroll"></textarea>
<div id="scrollableDiv" style="overflow:scroll"></div>

<script>
const scrollbarsHaveThickness = (scrollableDiv.offsetHeight != 0);
test(() => {
  assert_equals(rows1.offsetWidth, rows1Hidden.offsetWidth);
  assert_equals(rows1.offsetHeight, rows1Hidden.offsetHeight);
}, 'rows=1 doesnt include scrollbar thickness');

test(() => {
  assert_equals(rows1.offsetWidth, rows1Scroll.offsetWidth);
  if (scrollbarsHaveThickness) {
    assert_less_than(rows1.offsetHeight, rows1Scroll.offsetHeight);
  } else {
    assert_equals(rows1.offsetHeight, rows1Scroll.offsetHeight);
  }
}, 'rows=1 overflow scroll includes scrollbar thickness');

test(() => {
  assert_equals(cols3.offsetWidth, cols3Scroll.offsetWidth);
  if (scrollbarsHaveThickness) {
    assert_greater_than(cols3.offsetWidth, cols3Hidden.offsetWidth);
  } else {
    assert_equals(cols3.offsetWidth, cols3Hidden.offsetWidth);
  }
  assert_equals(cols3.offsetHeight, cols3Scroll.offsetHeight);
  assert_equals(cols3.offsetHeight, cols3Hidden.offsetHeight);
}, 'cols=3 includes scrollbar thickness');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 241,
        "byte_start": 168,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 241,
        "byte_start": 168,
        "col": 1,
        "line": 4
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
  "source_name": "html/rendering/widgets/textarea-scrollbar-sizing-001.html"
}
```
