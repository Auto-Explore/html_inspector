# html/browsers/history/the-location-interface/location-prototype-setting-cross-origin.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location-prototype-setting-cross-origin.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>[[SetPrototypeOf]] on a Location object should not allow changing its value: cross-origin</title>
<link rel="help" href="http://html.spec.whatwg.org/multipage/#location-setprototypeof">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/test-setting-immutable-prototype.js"></script>

<iframe src="//{{domains[www]}}:{{ports[http][1]}}/common/blank.html"></iframe>

<script>
"use strict";
setup({ explicit_done: true });

window.onload = () => {
  const targetLocation = frames[0].location;

  test(() => {
    assert_equals(Object.getPrototypeOf(targetLocation), null);
  }, "Cross-origin: the prototype is null");

  testSettingImmutablePrototype("Cross-origin", targetLocation, null, { isSameOriginDomain: false });

  done();
};
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
  "source_name": "html/browsers/history/the-location-interface/location-prototype-setting-cross-origin.sub.html"
}
```
