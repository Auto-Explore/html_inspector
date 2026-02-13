# html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-rejection-events-iframe.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-rejection-events-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title></title>
<div id="log"></div><br>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
'use strict';

setup({
  allow_uncaught_exception: true
});

async_test(function(t) {
  createIframeAndStartTest(t, function(w) {
    let e = new Error();
    let promise = new w.Promise(function(_, reject) {
      setTimeout(function() {
        reject(e);
      }, 1);
    });

    let unhandled = function(evt) {
      if (evt.promise === promise) {
        t.step(function() {
          assert_equals(evt.reason, e);
          assert_equals(evt.promise, promise);
        });
        t.done();
      }
    };
    let handled = function(evt) {
      if (evt.promise === promise) {
        t.step(function() {
          assert_unreached('rejectionhandled event is not supposed to be triggered');
        });
      }
    };

    w.addEventListener('unhandledrejection', unhandled);
    w.addEventListener('rejectionhandled', handled);
    ensureCleanup(t, w, unhandled, handled);
  });
}, "unhandledrejection: promise is created in iframe and being rejected elsewhere");

async_test(function(t) {
  createIframeAndStartTest(t, function(w) {
    let e = new Error();
    let promise = w.Promise.reject(e);

    let unhandled = function(evt) {
      if (evt.promise === promise) {
        t.step(function() {
          assert_unreached('unhandledrejection event is not supposed to be triggered');
        });
      }
    };
    let handled = function(evt) {
      if (evt.promise === promise) {
        t.step(function() {
          assert_unreached('rejectionhandled event is not supposed to be triggered');
        });
      }
    };

    w.addEventListener('unhandledrejection', unhandled);
    w.addEventListener('rejectionhandled', handled);
    ensureCleanup(t, w, unhandled, handled);

    promise.catch(function() {});
    setTimeout(function() {
      t.done();
    }, 10);
  });
}, 'no unhandledrejection/rejectionhandled: promise is created in iframe and being rejected elsewhere');

async_test(function(t) {
  createIframeAndStartTest(t, function(w) {
    let e = new Error();
    let promise = w.Promise.reject(e);
    var unhandledPromises = [];
    var unhandledReasons = [];
    var handledPromises = [];
    var handledReasons = [];

    let unhandled = function(evt) {
      if (evt.promise === promise) {
        t.step(function() {
          unhandledPromises.push(evt.promise);
          unhandledReasons.push(evt.reason);

          setTimeout(function() {
            var unreached = t.unreached_func('promise should not be fulfilled');
            promise.then(unreached, function(reason) {
              assert_equals(reason, e);
              setTimeout(function() {
                assert_array_equals(handledPromises, [promise]);
                assert_array_equals(handledReasons, [e]);
                t.done();
              }, 10);
            });
          }, 10);
        });
      }
    };
    let handled = function(evt) {
      if (evt.promise === promise) {
        t.step(function() {
          assert_array_equals(unhandledPromises, [promise]);
          assert_array_equals(unhandledReasons, [e]);
          handledPromises.push(evt.promise);
          handledReasons.push(evt.reason);
        });
      }
    };

    w.addEventListener('unhandledrejection', unhandled);
    w.addEventListener('rejectionhandled', handled);
    ensureCleanup(t, w, unhandled, handled);
  });
}, 'delayed handling: promise is created in iframe and being rejected elsewhere');

// Helpers

function createIframeAndStartTest(t, runTest) {
  var iframe = document.createElement("iframe");
  iframe.onload = function() {
    t.add_cleanup(() => iframe.remove());
    runTest(iframe.contentWindow);
  };
  iframe.srcdoc = '';
  document.documentElement.appendChild(iframe);
}

function ensureCleanup(t, win, unhandled, handled) {
  t.add_cleanup(function() {
    if (unhandled) {
      win.removeEventListener('unhandledrejection', unhandled);
    }
    if (handled) {
      win.removeEventListener('rejectionhandled', handled);
    }
  });
}

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.title.empty",
      "message": "Element “title” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 44,
        "byte_start": 37,
        "col": 1,
        "line": 3
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
  "source_name": "html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-rejection-events-iframe.html"
}
```
