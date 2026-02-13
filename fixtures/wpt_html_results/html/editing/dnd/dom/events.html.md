# html/editing/dnd/dom/events.html

Counts:
- errors: 2
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/dom/events.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset='utf-8'>
<title>drag &amp; drop – events</title>

<script src='/resources/testharness.js'></script>
<script src='/resources/testharnessreport.js'></script>


<noscript><p>Enable JavaScript and reload.</p></noscript>

<div id='log'></div>

<script>

var element = document.createElement('div');

test(function () {
  assert_equals(element.ondragstart, null);
}, 'element.ondragstart initial value');

test(function () {
  assert_equals(element.ondrag, null);
}, 'element.ondrag must initial value');

test(function () {
  assert_equals(element.ondragenter, null);
}, 'element.ondragenter initial value');

test(function () {
  assert_equals(element.ondragleave, null);
}, 'element.ondragleave initial value');

test(function () {
  assert_equals(element.ondragover, null);
}, 'element.ondragover initial value');

test(function () {
  assert_equals(element.ondrop, null);
}, 'element.ondrop initial value');

test(function () {
  assert_equals(element.ondragend, null);
}, 'element.ondragend initial value');

</script>

</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “body”.",
      "severity": "Error",
      "span": {
        "byte_end": 1060,
        "byte_start": 1053,
        "col": 1,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 1068,
        "byte_start": 1061,
        "col": 1,
        "line": 48
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
  "source_name": "html/editing/dnd/dom/events.html"
}
```
