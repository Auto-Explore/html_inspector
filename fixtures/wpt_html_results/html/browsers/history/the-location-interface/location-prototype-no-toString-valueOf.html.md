# html/browsers/history/the-location-interface/location-prototype-no-toString-valueOf.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location-prototype-no-toString-valueOf.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Location.prototype objects don't have own "toString" and "valueOf" properties</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
"use strict";

test(t => {
  assert_not_own_property(Location.prototype, "toString");
  t.add_cleanup(() => { delete Location.prototype.toString; });

  let val;
  Object.defineProperty(Location.prototype, "toString", {
    get: () => val,
    set: newVal => { val = newVal; },
    enumerable: false,
    configurable: true,
  });

  Location.prototype.toString = 2;
  assert_equals(Location.prototype.toString, 2);
}, "'toString' accessor property is defined");

test(t => {
  assert_not_own_property(Location.prototype, "toString");
  t.add_cleanup(() => { delete Location.prototype.toString; });

  Location.prototype.toString = 4;
  assert_equals(Location.prototype.toString, 4);
}, "'toString' data property is created via [[Set]]");

test(t => {
  assert_not_own_property(Location.prototype, "valueOf");
  t.add_cleanup(() => { delete Location.prototype.valueOf; });

  Object.defineProperty(Location.prototype, "valueOf", {
    get: () => 6,
    enumerable: true,
    configurable: true,
  });

  assert_equals(Location.prototype.valueOf, 6);
}, "'valueOf' accessor property is defined");

test(t => {
  assert_not_own_property(Location.prototype, "valueOf");
  t.add_cleanup(() => { delete Location.prototype.valueOf; });

  Location.prototype.valueOf = 8;
  assert_equals(Object.getOwnPropertyDescriptor(Location.prototype, "valueOf").value, 8);
}, "'valueOf' data property is created via [[Set]]");
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
  "source_name": "html/browsers/history/the-location-interface/location-prototype-no-toString-valueOf.html"
}
```
