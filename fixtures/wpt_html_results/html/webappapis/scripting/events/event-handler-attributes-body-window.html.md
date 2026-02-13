# html/webappapis/scripting/events/event-handler-attributes-body-window.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-attributes-body-window.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTMLBodyElement event handlers</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/WebIDLParser.js"></script>
<script src="resources/event-handler-body.js"></script>

<body>
<script>
setup({ explicit_done: true });

handlersListPromise.then(({ shadowedHandlers, notShadowedHandlers }) => {
  eventHandlerTest(shadowedHandlers, notShadowedHandlers, "body");

  done();
});
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
  "source_name": "html/webappapis/scripting/events/event-handler-attributes-body-window.html"
}
```
