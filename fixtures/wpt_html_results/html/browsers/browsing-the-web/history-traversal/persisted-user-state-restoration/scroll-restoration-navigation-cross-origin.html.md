# html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/scroll-restoration-navigation-cross-origin.html

Counts:
- errors: 5
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/scroll-restoration-navigation-cross-origin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name=timeout content=long>
<title>Correct behaviour of scroll restoration mode is cross origin history traversal</title>

<style>
  iframe {
    height: 300px;
    width: 300px;
  }
</style>

<body>
  <iframe></iframe>
</body>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script type="text/javascript">
  'use strict';

  // The test does the following navigation steps for iframe
  // 1. load blank1
  // 2. load blank2
  // 3. go back to blank1
  async_test(function(t) {
    var iframe = document.querySelector('iframe');
    var baseURL = location.href.substring(0, location.href.lastIndexOf('/'));

    var steps = [
      function() {
        iframe.src = 'resources/blank1.html';
      },
      function() {
        assert_equals(iframe.contentWindow.location.href, baseURL + '/resources/blank1.html', 'should be on first blank page');
        iframe.contentWindow.history.scrollRestoration = 'manual';
        assert_equals(iframe.contentWindow.history.scrollRestoration, 'manual');
        iframe.contentWindow.scrollTo(500, 500);
        assert_equals(iframe.contentWindow.scrollX, 500, 'scripted scrolling should take effect');
        assert_equals(iframe.contentWindow.scrollY, 500, 'scripted scrolling should take effect');
        setTimeout(next, 0);
      },
      function() {
        // navigate to new page
        iframe.src = 'resources/blank2.html';
      },
      function() {
        assert_equals(iframe.contentWindow.location.href, baseURL + '/resources/blank2.html', 'should be on second blank page');
        assert_equals(iframe.contentWindow.history.scrollRestoration, 'auto', 'new page loads should set scrollRestoration to "auto"');
        setTimeout(next, 0);
      }, function() {
        iframe.contentWindow.history.back();
      }, function() {
        // coming back scrollRestoration should be restored to 'manual' and respected
        assert_equals(iframe.contentWindow.location.href, baseURL + '/resources/blank1.html', 'should be back on first blank page');
        assert_equals(iframe.contentWindow.history.scrollRestoration, 'manual', 'navigating back should retain scrollRestoration value');
        assert_equals(iframe.contentWindow.scrollX, 0, 'horizontal scroll offset should not be restored');
        assert_equals(iframe.contentWindow.scrollY, 0, 'vertical scroll offset should not be restored');
        t.done();
      }
    ];

    var stepCount = 0;
    var next = t.step_func(function() {
      steps[stepCount++]();
    });

    iframe.onload = next;
    next();
  }, 'Navigating to new page should reset to "auto" and navigating back should restore and respect scroll restoration mode');

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 290,
        "byte_start": 250,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 299,
        "byte_start": 290,
        "col": 41,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 355,
        "byte_start": 346,
        "col": 47,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 387,
        "byte_start": 356,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 2737,
        "byte_start": 387,
        "col": 32,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 2746,
        "byte_start": 2737,
        "col": 1,
        "line": 71
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/scroll-restoration-navigation-cross-origin.html"
}
```
