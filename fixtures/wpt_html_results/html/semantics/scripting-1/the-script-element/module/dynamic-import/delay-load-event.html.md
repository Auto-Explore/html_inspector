# html/semantics/scripting-1/the-script-element/module/dynamic-import/delay-load-event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/delay-load-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Dynamic imports don't delay the load event</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
// Dynamic imports don't #delay-the-load-event.
// Therefore, Window load event would be fired
// just after the dynamic import() below starts.
window.loaded = [];
window.addEventListener('load', () => loaded.push('Window load event'));
promise_test(t => {
  loaded.push('import start');
  // This 'loading' log is added to detect the previous Chromium behavior
  // where the Window load event is delayed until just before script
  // element's load event.
  t.step_timeout(() => loaded.push('loading'), 1000);
  return import("../resources/slow-module.js?pipe=trickle(d2)")
    .then(() => {
        assert_array_equals(
            loaded,
            ['import start', 'Window load event', 'loading', 'slow'],
            "Window load event shouldn't be delayed by dynamic imports");
      });
}, "Dynamic imports don't delay the load event.");
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/delay-load-event.html"
}
```
