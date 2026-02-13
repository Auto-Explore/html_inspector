# html/infrastructure/common-dom-interfaces/collections/historical.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/common-dom-interfaces/collections/historical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Historical HTML*Collection features should not be supported</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<form id=form><input name=foo></form>
<select id=select><option name=bar></select>
<div id=dupe>
<div id=dupe>
<script>
test(function() {
  var collection = document.getElementById('form').elements;
  assert_equals(typeof collection, 'object', 'typeof');
  assert_throws_js(TypeError, function() {
    collection('foo');
  });
}, 'HTMLFormControlsCollection legacycaller should not be supported');

test(function() {
  var collection = document.getElementById('select').options;
  assert_equals(typeof collection, 'object', 'typeof');
  assert_throws_js(TypeError, function() {
    collection('bar');
  });
}, 'HTMLOptionsCollection legacycaller should not be supported');

test(function() {
  var collection = document.all('dupe', 0);
  // If the second argument were used, it would return the first item of the
  // collection instead of the whole collection.
  assert_equals(collection.length, 2, 'length');
}, 'HTMLAllCollection legacycaller with two arguments should not be supported');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “dupe”.",
      "severity": "Error",
      "span": {
        "byte_end": 326,
        "byte_start": 313,
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
  "source_name": "html/infrastructure/common-dom-interfaces/collections/historical.html"
}
```
