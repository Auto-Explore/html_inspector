# html/browsers/the-window-object/window-prototype-chain.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/window-prototype-chain.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Prototype chain of the window object</title>
<link rel="author" title="Ms2ger" href="ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#window">
<link rel="help" href="https://dom.spec.whatwg.org/#eventtarget">
<link rel="help" href="https://webidl.spec.whatwg.org/#interface-prototype-object">
<link rel="help" href="https://webidl.spec.whatwg.org/#named-properties-object">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
test(function() {
  assert_class_string(window, "Window");
}, "window object");
test(function() {
  var proto = Object.getPrototypeOf(window);
  assert_equals(proto, Window.prototype);
}, "Window.prototype");
test(function() {
  var gsp = Object.getPrototypeOf(Object.getPrototypeOf(window));
  assert_class_string(gsp, "WindowProperties");
}, "Global scope polluter");
test(function() {
  var protoproto = Object.getPrototypeOf(Object.getPrototypeOf(Object.getPrototypeOf(window)));
  assert_equals(protoproto, EventTarget.prototype);
}, "EventTarget.prototype");
test(function() {
  var protoprotoproto = Object.getPrototypeOf(Object.getPrototypeOf(Object.getPrototypeOf(Object.getPrototypeOf(window))));
  assert_equals(protoprotoproto, Object.prototype);
}, "Object.prototype");
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
  "source_name": "html/browsers/the-window-object/window-prototype-chain.html"
}
```
