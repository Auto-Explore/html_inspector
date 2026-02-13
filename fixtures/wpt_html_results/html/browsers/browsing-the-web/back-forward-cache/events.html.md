# html/browsers/browsing-the-web/back-forward-cache/events.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/back-forward-cache/events.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="resources/helper.sub.js"></script>
<script>
// Basic event tests.
runEventTest(
  {targetOrigin: originSameOrigin},
  'SameOrigin');

runEventTest(
  {targetOrigin: originSameSite},
  'SameSite');

runEventTest(
  {},
  'CrossSite');

// beforeunload.
runEventTest({
  events: ['pagehide', 'pageshow', 'load', 'beforeunload'],
  expectedEvents: [
    'window.load',
    'window.pageshow',
    'window.beforeunload',
    'window.pagehide.persisted',
    'window.pageshow.persisted'
  ]},
  'beforeunload');

// unload.
runEventTest({
  events: ['pagehide', 'pageshow', 'load', 'unload'],
  expectedEvents: [
    'window.load',
    'window.pageshow',
    'window.pagehide.persisted',
    'window.pageshow.persisted'
  ]},
  'unload');
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
  "source_name": "html/browsers/browsing-the-web/back-forward-cache/events.html"
}
```
