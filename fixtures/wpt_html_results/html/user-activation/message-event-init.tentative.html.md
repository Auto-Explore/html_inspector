# html/user-activation/message-event-init.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/message-event-init.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<!--
   Tentative due to:
   https://github.com/whatwg/html/issues/1983
-->
<title>MessageEvent constructor</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(function() {
  var ev = new MessageEvent("test", { userActivation: navigator.userActivation })
  assert_equals(ev.userActivation, navigator.userActivation, "userActivation attribute")
}, "MessageEventInit user activation set")
test(function() {
  var ev = new MessageEvent("test")
  assert_equals(ev.userActivation, null, "userActivation attribute")
}, "MessageEventInit user activation not set")

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
  "source_name": "html/user-activation/message-event-init.tentative.html"
}
```
