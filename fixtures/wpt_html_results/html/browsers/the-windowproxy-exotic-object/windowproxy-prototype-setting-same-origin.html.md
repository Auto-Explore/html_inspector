# html/browsers/the-windowproxy-exotic-object/windowproxy-prototype-setting-same-origin.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-windowproxy-exotic-object/windowproxy-prototype-setting-same-origin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>[[SetPrototypeOf]] on a WindowProxy object should not allow changing its value: same-origin</title>
<link rel="help" href="http://html.spec.whatwg.org/multipage/#windowproxy-setprototypeof">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/test-setting-immutable-prototype.js"></script>

<script>
"use strict";

const origProto = Object.getPrototypeOf(window);

test(() => {
  assert_not_equals(origProto, null);
}, "Same-origin prerequisite check: the original prototype is accessible");

testSettingImmutablePrototype("Same-origin", window, origProto, { isSameOriginDomain: true });
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
  "source_name": "html/browsers/the-windowproxy-exotic-object/windowproxy-prototype-setting-same-origin.html"
}
```
