# html/webappapis/timers/setinterval-cross-realm-callback-report-exception.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/timers/setinterval-cross-realm-callback-report-exception.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>window.setInterval() reports the exception from its callback in the callback's global object</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<iframe></iframe>
<iframe></iframe>
<iframe></iframe>
<script>
setup({ allow_uncaught_exception: true });

const onerrorCalls = [];
window.onerror = () => { onerrorCalls.push("top"); };
frames[0].onerror = () => { onerrorCalls.push("frame0"); };
frames[1].onerror = () => { onerrorCalls.push("frame1"); };
frames[2].onerror = () => { onerrorCalls.push("frame2"); };

async_test(t => {
  window.onload = t.step_func(() => {
    const id = frames[0].setInterval(new frames[1].Function(`
      parent.clearThisInterval();
      throw new parent.frames[2].Error("PASS");
    `), 4);
    window.clearThisInterval = () => { frames[0].clearInterval(id); };

    t.step_wait_func_done(() => onerrorCalls.length > 0,
                          () => assert_array_equals(onerrorCalls, ["frame1"]),
                          undefined, 1000, 10);
  });
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
  "source_name": "html/webappapis/timers/setinterval-cross-realm-callback-report-exception.html"
}
```
