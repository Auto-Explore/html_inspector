# html/browsers/the-window-object/named-access-on-the-window-object/prototype.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/named-access-on-the-window-object/prototype.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Named access with shadowing properties</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
test(function() {
  var name = "named1";
  window[name] = "shadowing";
  var element = document.createElement("span");
  element.id = name;
  document.body.appendChild(element);

  assert_equals(window[name], "shadowing");
  assert_equals(Object.getOwnPropertyDescriptor(window, name).value, "shadowing");

  assert_equals(Window.prototype[name], element);
  assert_equals(Object.getOwnPropertyDescriptor(Window.prototype, name), undefined);

  var npo = Object.getPrototypeOf(Window.prototype);
  assert_equals(npo[name], element);
  assert_equals(Object.getOwnPropertyDescriptor(npo, name).value, element);

  assert_equals(EventTarget.prototype[name], undefined);
  assert_equals(Object.getOwnPropertyDescriptor(EventTarget.prototype, name), undefined);
}, "Property on window.");

test(function() {
  var name = "named2";
  Window.prototype[name] = "shadowing";
  var element = document.createElement("span");
  element.id = name;
  document.body.appendChild(element);

  assert_equals(window[name], "shadowing");
  assert_equals(Object.getOwnPropertyDescriptor(window, name), undefined);

  assert_equals(Window.prototype[name], "shadowing");
  assert_equals(Object.getOwnPropertyDescriptor(Window.prototype, name).value, "shadowing");

  var npo = Object.getPrototypeOf(Window.prototype);
  assert_equals(npo[name], element);
  assert_equals(Object.getOwnPropertyDescriptor(npo, name).value, element);

  assert_equals(EventTarget.prototype[name], undefined);
  assert_equals(Object.getOwnPropertyDescriptor(EventTarget.prototype, name), undefined);
}, "Property on Window.prototype.");

test(function() {
  var name = "named3";
  EventTarget.prototype[name] = "shadowing";
  var element = document.createElement("span");
  element.id = name;
  document.body.appendChild(element);

  assert_equals(window[name], "shadowing");
  assert_equals(Object.getOwnPropertyDescriptor(window, name), undefined);

  assert_equals(Window.prototype[name], "shadowing");
  assert_equals(Object.getOwnPropertyDescriptor(Window.prototype, name), undefined);

  var npo = Object.getPrototypeOf(Window.prototype);
  assert_equals(npo[name], "shadowing");
  assert_equals(Object.getOwnPropertyDescriptor(npo, name), undefined);

  assert_equals(EventTarget.prototype[name], "shadowing");
  assert_equals(Object.getOwnPropertyDescriptor(EventTarget.prototype, name).value, "shadowing");
}, "Property on EventTarget.prototype.");

test(function() {
  var name = "named4";
  Object.prototype[name] = "shadowing";
  var element = document.createElement("span");
  element.id = name;
  document.body.appendChild(element);

  assert_equals(window[name], "shadowing");
  assert_equals(Object.getOwnPropertyDescriptor(window, name), undefined);

  assert_equals(Window.prototype[name], "shadowing");
  assert_equals(Object.getOwnPropertyDescriptor(Window.prototype, name), undefined);

  var npo = Object.getPrototypeOf(Window.prototype);
  assert_equals(npo[name], "shadowing");
  assert_equals(Object.getOwnPropertyDescriptor(npo, name), undefined);

  assert_equals(EventTarget.prototype[name], "shadowing");
  assert_equals(Object.getOwnPropertyDescriptor(EventTarget.prototype, name), undefined);

  assert_equals(Object.prototype[name], "shadowing");
  assert_equals(Object.getOwnPropertyDescriptor(Object.prototype, name).value, "shadowing");
}, "Property on Object.prototype.");
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
  "source_name": "html/browsers/the-window-object/named-access-on-the-window-object/prototype.html"
}
```
