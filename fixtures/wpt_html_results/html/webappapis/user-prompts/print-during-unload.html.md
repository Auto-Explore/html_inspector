# html/webappapis/user-prompts/print-during-unload.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/user-prompts/print-during-unload.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>print() during unload</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
"use strict";

async_test(t => {
  const w = window.open("resources/print-during-event.sub.html?event=unload");
  t.add_cleanup(() => w.close());

  const messages = [];
  window.addEventListener("message", t.step_func(({ data }) => {
    messages.push(data);

    if (messages.length === 1) {
      assert_array_equals(messages, ["start"]);
      w.location.href = "resources/destination.html";
    } else if (messages.length === 2) {
      // The test passes if we've reached this point because the print() dialog did not block the navigation.
      assert_array_equals(messages, ["start", "destination"]);
      t.done();
    }
  }));
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
  "source_name": "html/webappapis/user-prompts/print-during-unload.html"
}
```
