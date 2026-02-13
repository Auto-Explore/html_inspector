# html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/resume-timer-on-history-back.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/resume-timer-on-history-back.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Verify history.back() on a persisted page resumes timers</title>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script type="text/javascript">

function make_post_back_url(name) {
  return new URL('resources/post_name_on_load.html?name=' + name,
                 window.location).href;
}

function wait_for_message(name) {
  return new Promise(resolve => {
    addEventListener('message', function onMsg(evt) {
      if (evt.data !== name) {
        return;
      }
      removeEventListener('message', onMsg);
      resolve();
    });
  });
}

function with_window_by_name(name) {
  let win = window.open(make_post_back_url(name));
  return wait_for_message(name).then(_ => {
    return win;
  });
}

function with_nested_frame(win, url) {
  return new Promise(resolve => {
    let frame = win.document.createElement('iframe');
    frame.addEventListener('load', function onLoad(evt) {
      removeEventListener('load', onLoad);
      resolve(frame);
    });
    frame.src = url;
    win.document.body.appendChild(frame);
  });
}

function delay(win, delay) {
  return new Promise(resolve => {
    win.setTimeout(_ => {
      resolve(win);
    }, delay);
  });
}

function navigate_by_name(win, name) {
  win.location = make_post_back_url(name);
  return wait_for_message(name).then(_ => {
    return win;
  });
}

function go_back(win) {
  return new Promise(resolve => {
    win.onpagehide = e => resolve(win);
    win.history.back();
  });
}

let DELAY = 500;

promise_test(t => {
  // Create a new window so we can navigate it later.
  return with_window_by_name('foo').then(win => {
    // Schedule a timer within the new window.  Our intent is
    // to navigate the window before the timer fires.
    let delayFired = false;
    let innerDelay = delay(win, DELAY);
    innerDelay.then(_ => {
      delayFired = true;
    });

    return navigate_by_name(win, 'bar').then(_ => {
      // Since the window has navigated the timer should not
      // fire.  We set a timer on our current test window
      // to verify the other timer is not received.
      assert_false(delayFired);
      return delay(window, DELAY * 2);
    }).then(_ => {
      // The navigated window's timer should not have fired.
      assert_false(delayFired);
      // Now go back to the document that set the timer.
      return go_back(win);
    }).then(_ => {
      // We wait for one of two conditions here.  For browsers
      // with a bfcache the original suspended timer will fire.
      // Alternatively, if the browser reloads the page the original
      // message will be sent again.  Wait for either of these
      // two events.
      return Promise.race([wait_for_message('foo'), innerDelay]);
    }).then(_ => {
      win.close();
    });
  });
}, 'history.back() handles top level page timer correctly');

promise_test(t => {
  let win;
  // Create a new window so we can navigate it later.
  return with_window_by_name('foo').then(w => {
    win = w;

    // Create a nested frame so we check if navigation and history.back()
    // properly handle child window state.
    return with_nested_frame(win, 'about:blank');

  }).then(frame => {
    // Schedule a timer within the nested frame contained by the new window.
    // Our intent is to navigate the window before the timer fires.
    let delayFired = false;
    let innerDelay = delay(frame.contentWindow, DELAY);
    innerDelay.then(_ => {
      delayFired = true;
    });

    return navigate_by_name(win, 'bar').then(_ => {
      // Since the window has navigated the timer should not
      // fire.  We set a timer on our current test window
      // to verify the other timer is not received.
      assert_false(delayFired);
      return delay(window, DELAY * 2);
    }).then(_ => {
      // The navigated window's timer should not have fired.
      assert_false(delayFired);
      // Now go back to the document containing the frame that set the timer.
      return go_back(win);
    }).then(_ => {
      // We wait for one of two conditions here.  For browsers
      // with a bfcache the original suspended timer will fire.
      // Alternatively, if the browser reloads the page the original
      // message will be sent again.  Wait for either of these
      // two events.
      return Promise.race([wait_for_message('foo'), innerDelay]);
    }).then(_ => {
      win.close();
    });
  });
}, 'history.back() handles nested iframe timer correctly');

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 262,
        "byte_start": 231,
        "col": 1,
        "line": 6
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/resume-timer-on-history-back.html"
}
```
