# html/webappapis/scripting/events/event-handler-non-content-document-idl-attributes.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-non-content-document-idl-attributes.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/multipage/webappapis.html#handler-onreadystatechange">
<link rel="help" href="https://html.spec.whatwg.org/multipage/webappapis.html#handler-onvisibilitychange">
<script>
  var handlerExecuted = false;
  ["div", "body", "frameset"].forEach(elementName => {
    ["readystatechange", "visibilitychange"].forEach(eventName => {
      let attributeName = `on${eventName}`;
      test(t => {
        t.add_cleanup(_ => { handlerExecuted = false });
        let element = document.createElement(elementName);
        element.setAttribute(attributeName, "handlerExecuted = true;")
        element.dispatchEvent(new Event(eventName, {bubbles: false}));
        assert_false(handlerExecuted);
      },`${elementName}.on${eventName} is not an event handler content attribute`);
    });
  });
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/webappapis/scripting/events/event-handler-non-content-document-idl-attributes.html"
}
```
