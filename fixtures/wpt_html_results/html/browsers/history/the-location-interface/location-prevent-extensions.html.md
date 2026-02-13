# html/browsers/history/the-location-interface/location-prevent-extensions.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location-prevent-extensions.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>[[PreventExtensions]] on a Location object should return false</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/history.html#location-preventextensions">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
"use strict";

test(() => {
  assert_throws_js(TypeError, () => {
    Object.preventExtensions(location);
  });
}, "Object.preventExtensions throws a TypeError");

test(() => {
  assert_false(Reflect.preventExtensions(location));
}, "Reflect.preventExtensions returns false");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/history/the-location-interface/location-prevent-extensions.html"
}
```
