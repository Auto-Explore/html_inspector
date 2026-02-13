# html/semantics/interestfor/interestfor-css-shorthands.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-css-shorthands.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/css/support/shorthand-testcommon.js"></script>
<script src="/css/support/parsing-testcommon.js"></script>

<body>
<script>
test_shorthand_value('interest-delay', '0.23s 450ms', {
  'interest-delay-start': '0.23s',
  'interest-delay-end': '450ms'
});
test_shorthand_value('interest-delay', '0.23s', {
  'interest-delay-start': '0.23s',
  'interest-delay-end': '0.23s'
});
test_shorthand_value('interest-delay', '450ms', {
  'interest-delay-start': '450ms',
  'interest-delay-end': '450ms'
});
test_shorthand_value('interest-delay', 'normal', {
  'interest-delay-start': 'normal',
  'interest-delay-end': 'normal'
});
test_shorthand_value('interest-delay', 'normal normal', {
  'interest-delay-start': 'normal',
  'interest-delay-end': 'normal'
});
test_shorthand_value('interest-delay', 'normal 0.23s', {
  'interest-delay-start': 'normal',
  'interest-delay-end': '0.23s'
});
test_shorthand_value('interest-delay', '0.23s normal', {
  'interest-delay-start': '0.23s',
  'interest-delay-end': 'normal'
});

test_invalid_value('interest-delay', '');
test_invalid_value('interest-delay', '0');
test_invalid_value('interest-delay', '0.23s 0.23s 0.23s');
test_invalid_value('interest-delay', 'normal normal normal');
test_invalid_value('interest-delay', '0.23s normal normal');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/interestfor/interestfor-css-shorthands.tentative.html"
}
```
