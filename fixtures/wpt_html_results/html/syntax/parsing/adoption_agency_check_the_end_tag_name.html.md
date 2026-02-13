# html/syntax/parsing/adoption_agency_check_the_end_tag_name.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/adoption_agency_check_the_end_tag_name.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>The adoption agency algorithm should check the end tag's name</title>
<link rel="author" href="mailto:n4ag3a2sh1i@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/parsing.html#adoption-agency-algorithm">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>


<script>
'use strict';

// This is a regression test for https://crbug.com/1217523.
test(() => {
  const wrapper = document.createElement('div');
  const html = '<code some-attribute=""><div><code><code><code><code></code></code></code></code></div></code>';
  wrapper.innerHTML = html;
  assert_equals(wrapper.innerHTML, html);
}, 'The algorithm should not reparent properly nested tags');

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
  "source_name": "html/syntax/parsing/adoption_agency_check_the_end_tag_name.html"
}
```
