# html/editing/the-hidden-attribute/hidden-until-found-text-fragment.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/hidden-until-found-text-fragment.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>beforematch fired on ScrollToTextFragment</title>
<link rel="author" title="Joey Arhar" href="mailto:jarhar@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#attr-hidden-until-found">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<script src="/common/utils.js"></script>
<script src="/scroll-to-text-fragment/stash.js"></script>

<script>
promise_test(t => new Promise((resolve, reject) => {
  const key = token();
  test_driver.bless('Open a scroll to text fragment URL', () => {
    window.open(
      `resources/hidden-until-found-text-fragment.html?key=${key}#:~:text=foo`,
      '_blank',
      'noopener');
  });
  fetchResults(key, resolve, reject);
}).then(results => {
  assert_false(results.targetHasHiddenAttribute,
    'The matching hidden=until-found element should have its hidden attribute removed so it can be scrolled to.');
  assert_true(results.pageYOffsetAfterRaf > 0,
    'The page should be scrolled down to foo.');
}), 'Verifies that the beforematch event is fired on the matching element of a ScrollToTextFragment navigation.');
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
  "source_name": "html/editing/the-hidden-attribute/hidden-until-found-text-fragment.html"
}
```
