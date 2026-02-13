# html/semantics/scripting-1/the-script-element/script-referrerpolicy-idl.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-referrerpolicy-idl.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>&lt;script> referrerPolicy IDL</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#referrer-policy-attribute">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
  test(() => {
    const script = document.createElement('script');
    document.body.appendChild(script);
    assert_equals(script.referrerPolicy,"",'Missing content attribute should reflect as empty');
    script.setAttribute('referrerpolicy','no-referrer');
    assert_equals(script.referrerPolicy,"no-referrer",'Valid value should reflect');
    script.setAttribute('referrerpolicy','');
    assert_equals(script.referrerPolicy,"",'Empty string should reflect as empty');
    script.setAttribute('referrerpolicy','invalid-value-here');
    assert_equals(script.referrerPolicy,"",'Invalid values should reflect as empty');
    script.referrerPolicy = 'no-referrer';
    assert_equals(script.referrerPolicy,"no-referrer",'Valid value via IDL');
    script.referrerPolicy = null;
    assert_equals(script.referrerPolicy,"",'Null should reflect as empty');
  },'Missing/invalid/null referrerPolicy should reflect as the empty string')
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-referrerpolicy-idl.html"
}
```
