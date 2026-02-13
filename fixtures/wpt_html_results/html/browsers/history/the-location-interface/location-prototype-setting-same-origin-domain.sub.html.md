# html/browsers/history/the-location-interface/location-prototype-setting-same-origin-domain.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location-prototype-setting-same-origin-domain.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>[[SetPrototypeOf]] on a Location object should not allow changing its value: cross-origin, but same-origin-domain</title>
<link rel="help" href="http://html.spec.whatwg.org/multipage/#location-setprototypeof">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/test-setting-immutable-prototype.js"></script>

<iframe src="//{{domains[www]}}:{{ports[http][1]}}/common/domain-setter.sub.html"></iframe>

<script>
"use strict";
document.domain = "{{host}}";
setup({ explicit_done: true });

window.onload = () => {
  const targetLocation = frames[0].location;
  const origProto = Object.getPrototypeOf(targetLocation);

  test(() => {
    assert_not_equals(origProto, null);
  }, "Same-origin-domain prerequisite check: the original prototype is accessible");

  testSettingImmutablePrototype("Same-origin-domain", targetLocation, origProto, { isSameOriginDomain: true }, frames[0]);

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
  "source_name": "html/browsers/history/the-location-interface/location-prototype-setting-same-origin-domain.sub.html"
}
```
