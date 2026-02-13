# html/semantics/forms/form-submission-target/form-target-request-header.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-target/form-target-request-header.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Form request header test</title>
<script src="/common/utils.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
async_test(t => {
  window.addEventListener("load", function() {
    let form = document.createElement("form");
    form.action = "resources/form-target-request-header-helper.py";
    form.method = "post";
    form.target = "_blank";

    const channelName = token();
    const channel = new BroadcastChannel(channelName);
    channel.onmessage = t.step_func_done(e => {
      assert_equals(e.data, "OK");
    });

    let url_input = document.createElement("input");
    url_input.type = "hidden";
    url_input.name = "channelname";
    url_input.value = channelName;

    form.appendChild(url_input);
    document.body.appendChild(form);
    form.submit();
  });
}, 'Verify the content-type exist during a form submission toward "_blank"');
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
  "source_name": "html/semantics/forms/form-submission-target/form-target-request-header.html"
}
```
