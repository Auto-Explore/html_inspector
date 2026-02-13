# html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/scroll-restoration-navigation-samedoc.html

Counts:
- errors: 5
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/scroll-restoration-navigation-samedoc.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Correct behaviour of scroll restoration mode in same document history traversals</title>

<style>
  body {
    height: 10000px;
    width: 10000px;
  }
</style>

<body></body>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script type="text/javascript">
  'use strict';

  async_test(function(t) {
    history.scrollRestoration = 'auto';
    window.scrollTo(0, 0);

    // create history entries and then verify the impact of scrollRestoration
    // when they are popped
    var entries = {
      /* For scroll restoration mode 'auto', the spec does not require scroll
         position to be restored at any particular value. */
      '#1': {type: 'push',    expectedScroll: null,  scrollRestoration: 'auto'},
      '#2': {type: 'replace', expectedScroll: null, scrollRestoration: 'auto'},
      /* For scroll restoration mode 'manual', the spec requires scroll position
         not to be restored. So we expect [555,555] which is the latest position
         before navigation. */
      '#3': {type: 'push',    expectedScroll: [555, 555], scrollRestoration: 'manual'},
      '#4': {type: 'replace', expectedScroll: [555, 555], scrollRestoration: 'manual'}
    };

    // setup entries
    for (var key in entries) {
      var entry = entries[key],
        beforeValue = history.scrollRestoration,
        newValue = entry.scrollRestoration;

      var args = [{key: key}, '', key];
      if (entry.type == 'push') {
        history.pushState.apply(history, args);
      } else {
        history.pushState(null, '', key);
        history.replaceState.apply(history, args);
      }
      assert_equals(history.scrollRestoration, beforeValue, `history.scrollRestoration value is retained after pushing new state`);
      history.scrollRestoration = newValue;
      assert_equals(history.scrollRestoration, newValue, `Setting scrollRestoration to ${newValue} works as expected`);
      window.scrollBy(50, 100);
    }

    // setup verification
    window.addEventListener('hashchange', t.step_func(function() {
      var key = location.hash,
        entry = entries[key];

      if (key === '') {
        t.done();
        return;
      }
      assert_equals(history.state.key, key, `state should have key: ${key}`);
      assert_equals(history.scrollRestoration, entry.scrollRestoration, 'scrollRestoration is updated correctly');
      if (entry.expectedScroll) {
        assert_equals(window.scrollX, entry.expectedScroll[0], `scrollX is correct for ${key}`);
        assert_equals(window.scrollY, entry.expectedScroll[1], `scrollY is correct for ${key}`);
      }

      window.history.back();
    }));

    // reset the scroll and kick off the verification
    setTimeout(function() {
      history.pushState(null, null, '#done');
      window.scrollTo(555, 555);
      window.history.back();
    }, 0);

  }, 'history.{push,replace}State retain scroll restoration mode and navigation in the same document respects it');
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
        "byte_end": 239,
        "byte_start": 199,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 248,
        "byte_start": 239,
        "col": 41,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 304,
        "byte_start": 295,
        "col": 47,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 336,
        "byte_start": 305,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 3010,
        "byte_start": 336,
        "col": 32,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 3019,
        "byte_start": 3010,
        "col": 1,
        "line": 81
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/scroll-restoration-navigation-samedoc.html"
}
```
