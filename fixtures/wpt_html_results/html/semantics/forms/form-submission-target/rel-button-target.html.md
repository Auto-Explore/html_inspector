# html/semantics/forms/form-submission-target/rel-button-target.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-target/rel-button-target.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>&lt;form rel> with &lt;button formtarget></title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src=resources/reltester.js></script>
<div id=log></div>
<form action=resources/endpoint.html><input type=hidden name=channelname><button type=submit formtarget=_blank></form>
<script>
const submitter = document.querySelector("button"),
      channelInput = document.querySelector("input");
relTester(submitter, channelInput, "<button formtarget>");
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
  "source_name": "html/semantics/forms/form-submission-target/rel-button-target.html"
}
```
