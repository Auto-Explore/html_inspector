# html/webappapis/scripting/processing-model-2/window-onerror-with-cross-frame-event-listeners-5.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/window-onerror-with-cross-frame-event-listeners-5.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>window.onerror listener reports the exception in global object of its callback</title>
<link rel=help href="https://dom.spec.whatwg.org/#concept-event-listener-inner-invoke">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<iframe></iframe>
<iframe></iframe>
<iframe></iframe>
<script>
setup({ allow_uncaught_exception: true });

window.onload = () => {
  test(() => {
    window.onerrorCalls = [];
    window.onerror = () => { onerrorCalls.push("top"); };
    frames[0].onerror = new frames[1].Function(`top.onerrorCalls.push("frame0"); throw new parent.frames[2].Error("PASS");`);
    frames[1].onerror = () => { onerrorCalls.push("frame1"); };
    frames[2].onerror = () => { onerrorCalls.push("frame2"); };

    frames[0].dispatchEvent(new ErrorEvent("error", { error: new Error("foo") }));
    assert_array_equals(onerrorCalls, ["frame0", "frame1"]);
  });
};
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
  "source_name": "html/webappapis/scripting/processing-model-2/window-onerror-with-cross-frame-event-listeners-5.html"
}
```
