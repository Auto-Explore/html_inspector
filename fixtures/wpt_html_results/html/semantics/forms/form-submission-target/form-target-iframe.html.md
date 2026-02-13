# html/semantics/forms/form-submission-target/form-target-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-target/form-target-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Form targetted at iframe</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
async_test(function(t) {
  window.addEventListener("load", t.step_func(function() {
    var frame = document.createElement("iframe");
    frame.name = "frame";
    document.documentElement.appendChild(frame);
    var form = document.createElement("form");
    form.target = "frame";
    form.action = "form-target-iframe-helper.py";
    form.method = "POST";
    var input = document.createElement("input");
    input.name = "n";
    form.appendChild(input);
    document.documentElement.appendChild(form);
    form.submit();
    frame.addEventListener("load", t.step_func(function() {
      if (frame.contentWindow.location.href.includes("form-target-iframe-helper.py")) {
        assert_equals(frame.contentWindow.document.body.textContent, "OK");
        t.done();
      }
    }));
  }));
}, "Form targetted at iframe");
</script>
<body>
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
  "source_name": "html/semantics/forms/form-submission-target/form-target-iframe.html"
}
```
