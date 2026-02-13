# html/webappapis/scripting/events/event-handler-attributes-frameset-window.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-attributes-frameset-window.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>event handlers</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/WebIDLParser.js"></script>
<script src="resources/event-handler-body.js"></script>
<script>
setup({ explicit_done: true });

handlersListPromise.then(({ shadowedHandlers, notShadowedHandlers }) => {
  eventHandlerTest(shadowedHandlers, notShadowedHandlers, "frameset");

  // The testharness framework appends test results to document.body,
  // show test results in frame after test done.
  add_completion_callback(() => {
    const log_elem = document.getElementById("log");
    const frame_elem = document.querySelector("frame");
    if (log_elem) {
      frame_elem.contentDocument.body.innerHTML = log_elem.innerHTML;
    }
  });

  done();
});
</script>
<frameset>
  <frame src="/common/blank.html" />
</frameset>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.frameset.obsolete",
      "message": "The “frameset” element is obsolete. Use the “iframe” element and CSS instead, or use server-side includes.",
      "severity": "Warning",
      "span": {
        "byte_end": 929,
        "byte_start": 919,
        "col": 1,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.parse.self_closing.non_void",
      "message": "Self-closing syntax (“/>”) used on a non-void HTML element. Ignoring the slash and treating as a start tag.",
      "severity": "Error",
      "span": {
        "byte_end": 966,
        "byte_start": 932,
        "col": 3,
        "line": 29
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/webappapis/scripting/events/event-handler-attributes-frameset-window.html"
}
```
