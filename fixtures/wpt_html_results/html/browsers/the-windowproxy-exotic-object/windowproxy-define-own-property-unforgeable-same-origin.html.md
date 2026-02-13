# html/browsers/the-windowproxy-exotic-object/windowproxy-define-own-property-unforgeable-same-origin.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-windowproxy-exotic-object/windowproxy-define-own-property-unforgeable-same-origin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>[[DefineOwnProperty]] on a WindowProxy forwards to OrdinaryDefineOwnProperty for same-origin objects</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/window-object.html#windowproxy-defineownproperty">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
"use strict";

for (const key of ["window", "document", "location", "top"]) {
    const { get, set } = Object.getOwnPropertyDescriptor(window, key);

    test(() => {
        Object.defineProperty(window, key, {});
        assert_true(Reflect.defineProperty(window, key, { configurable: false }), "[[Configurable]]: false");
        Object.defineProperty(window, key, { enumerable: true });

        assert_true(Reflect.defineProperty(window, key, { get }), "[[Get]]: unchanged");
        Object.defineProperty(window, key, { set });
        assert_true(Reflect.defineProperty(window, key, { get, set }), "[[Get]]: unchanged, [[Set]]: unchanged");

        Object.defineProperty(window, key, { get, set, enumerable: true, configurable: false });
    }, `[[DefineOwnProperty]] success: "${key}"`);

    test(() => {
        assert_throws_js(TypeError, () => {
            Object.defineProperty(window, key, { configurable: true });
        }, "[[Configurable]]: true");

        assert_false(Reflect.defineProperty(window, key, { enumerable: false }), "[[Enumerable]]: false");

        assert_throws_js(TypeError, () => {
            Object.defineProperty(window, key, { get() {}, set });
        }, "[[Get]]: changed, [[Set]]: unchanged");

        assert_false(Reflect.defineProperty(window, key, { get, set() {} }), "[[Get]]: unchanged, [[Set]]: changed");

        assert_throws_js(TypeError, () => {
            Object.defineProperty(window, key, { writable: false, configurable: true });
        }, "[[Writable]]: false, [[Configurable]]: true");

        assert_false(Reflect.defineProperty(window, key, { value: window[key], enumerable: true }), "[[Value]], [[Enumerable]]: true");
    }, `[[DefineOwnProperty]] failure: "${key}"`);
}
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
  "source_name": "html/browsers/the-windowproxy-exotic-object/windowproxy-define-own-property-unforgeable-same-origin.html"
}
```
