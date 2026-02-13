# html/browsers/the-windowproxy-exotic-object/windowproxy-prototype-setting-cross-origin-domain.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-windowproxy-exotic-object/windowproxy-prototype-setting-cross-origin-domain.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>[[SetPrototypeOf]] on a WindowProxy object should not allow changing its value: cross-origin via document.domain</title>
<link rel="help" href="http://html.spec.whatwg.org/multipage/#windowproxy-setprototypeof">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/test-setting-immutable-prototype.js"></script>

<iframe src="/common/domain-setter.sub.html"></iframe>

<script>
"use strict";
// This page does *not* set document.domain, so it's cross-origin with the iframe
setup({ explicit_done: true });

window.onload = () => {
  const target = frames[0];

  test(() => {
    assert_equals(Object.getPrototypeOf(target), null);
  }, "Cross-origin via document.domain: the prototype is null");

  testSettingImmutablePrototype("Cross-origin via document.domain", target, null, { isSameOriginDomain: false });

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
  "source_name": "html/browsers/the-windowproxy-exotic-object/windowproxy-prototype-setting-cross-origin-domain.sub.html"
}
```
