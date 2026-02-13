# html/browsers/history/the-location-interface/location-non-configurable-toString-valueOf.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location-non-configurable-toString-valueOf.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Same-origin Location objects have non-configurable "toString" and "valueOf" properties</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/history.html#location-defineownproperty">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
"use strict";

test(() => {
  assert_own_property(location, "toString");
  const origToString = location.toString;

  assert_throws_js(TypeError, () => {
    Object.defineProperty(location, "toString", {
      get() {},
      set(_v) {},
      enumerable: true,
      configurable: true,
    });
  });

  assert_equals(location.toString, origToString);
}, "'toString' redefinition with accessor fails");

test(() => {
  assert_own_property(location, "valueOf");
  const origValueOf = location.valueOf;

  assert_throws_js(TypeError, () => {
    Object.defineProperty(location, "valueOf", {
      get() {},
      enumerable: false,
      configurable: true,
    });
  });

  assert_equals(location.valueOf, origValueOf);
}, "'valueOf' redefinition with accessor fails");
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
  "source_name": "html/browsers/history/the-location-interface/location-non-configurable-toString-valueOf.html"
}
```
