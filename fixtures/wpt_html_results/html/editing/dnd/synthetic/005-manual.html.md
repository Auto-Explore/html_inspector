# html/editing/dnd/synthetic/005-manual.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/synthetic/005-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Synthetic events with real data store must inherit protection status from real events</title>
    <style type="text/css">
blockquote { height: 100px; width: 100px; background: orange; margin: 0; padding: 0; float: left; }
blockquote + blockquote { background: blue; }
blockquote + blockquote + blockquote { background: fuchsia; }
blockquote + div { clear: left; }
    </style>
    <script type="text/javascript" src="/resources/testharness.js"></script>
    <script type="text/javascript" src="/resources/testharnessreport.js"></script>
    <script type="text/javascript">
setup(function () {},{explicit_done:true,explicit_timeout:true});
window.onload = function () {

  var orange  = document.getElementsByTagName('blockquote')[0],
      blue    = document.getElementsByTagName('blockquote')[1],
      fuchsia = document.getElementsByTagName('blockquote')[2],
      evtdone = {};

  orange.ondragstart = function (e) {
    evtdone[e.type] = true;
    e.dataTransfer.effectAllowed = 'copy';

    var t = async_test(e.type+' should share its data with the synthetic event');
    blue.ondragstart = function (e) {
      t.step(function() {
        assert_equals( e.dataTransfer.getData('text'), 'dragstart real data', 'step 1' );
        e.dataTransfer.setData('text','dragstart-dragstart synthetic data');
        assert_equals( e.dataTransfer.getData('text'), 'dragstart-dragstart synthetic data', 'step 2' );
      });
    };
    t.step(function() {
      var evt = new DragEvent('dragstart', {dataTransfer:e.dataTransfer});
      e.dataTransfer.setData('text','dragstart real data'); //changing in between steps, just to make sure it uses the underlying data store, not a temporary clone
      blue.dispatchEvent(evt);
    });
    t.done();

    test(function() {
      assert_equals( e.dataTransfer.getData('text'), 'dragstart-dragstart synthetic data' );
    }, e.type+' should see the data from the synthetic event' );

    var t2 = async_test(e.type+' should share its protection status with the synthetic event');
    blue.ondrag = function (e) {
      t2.step(function() {
        e.dataTransfer.setData('text','dragstart-drag synthetic data');
        assert_equals( e.dataTransfer.getData('text'), 'dragstart-drag synthetic data' );
      });
    };
    t2.step(function() {
      var evt = new DragEvent('drag', {dataTransfer:e.dataTransfer});
      blue.dispatchEvent(evt);
    });
    t2.done();

    var t3 = async_test(e.type+' should share its protection status with the nested synthetic event');
    blue.ondrag = function (e) {
      blue.ondragend = function (e) {
        t3.step(function() {
          assert_equals( e.dataTransfer.getData('text'), 'dragstart-drag synthetic data', 'step1' );
          e.dataTransfer.setData('text','dragstart-drag-dragend synthetic data');
          assert_equals( e.dataTransfer.getData('text'), 'dragstart-drag-dragend synthetic data', 'step2' );
        });
      };
      t3.step(function() {
        var evt = new DragEvent('dragend', {dataTransfer:e.dataTransfer});
        blue.dispatchEvent(evt);
      });
    };
    t3.step(function() {
      var evt = new DragEvent('drag', {dataTransfer:e.dataTransfer});
      blue.dispatchEvent(evt);
    });
    t3.done();

    test(function() {
      assert_equals( e.dataTransfer.getData('text'), 'dragstart-drag-dragend synthetic data' );
    }, e.type+' should see the data from the nested synthetic event' );
  };

  blue.ondragenter = blue.ondragover = function (e) {
    e.preventDefault();
  };
  orange.ondrag = blue.ondragleave = function (e) {
    if( evtdone[e.type] ) { return; }
    evtdone[e.type] = true;
    var evtype = e.type;

    var t = async_test(e.type+' should share its data with the synthetic event');
    blue.ondragstart = function (e) {
      t.step(function() {
        assert_true( e.dataTransfer.items.length > 0, 'items.length' );
      });
    };
    t.step(function() {
   var evt = new DragEvent('dragstart', {dataTransfer:e.dataTransfer});
      blue.dispatchEvent(evt);
    });
    t.done();

    var t2 = async_test(e.type+' should share its protection status with the synthetic event');
    blue.ondragstart = function (e) {
      t2.step(function() {
        assert_equals( e.dataTransfer.getData('text'), '', 'step 1' );
        e.dataTransfer.setData('text',evtype+'-dragstart synthetic data');
        assert_equals( e.dataTransfer.getData('text'), '', 'step 2' );
      });
    };
    t2.step(function() {
   var evt = new DragEvent('dragstart', {dataTransfer:e.dataTransfer});
      blue.dispatchEvent(evt);
    });
    t2.done();

    test(function() {
      assert_equals( e.dataTransfer.getData('text'), '' );
    }, e.type+' protection status should not be modified by the synthetic event' );

    var t3 = async_test(e.type+' should share its protection status with the nested synthetic event');
    blue.ondragstart = function (e) {
      var div = document.createElement('div');
      div.ondragstart = function (e) {
        t3.step(function() {
          assert_equals( e.dataTransfer.getData('text'), '', 'step1' );
          e.dataTransfer.setData('text',evtype+'dragstart-dragstart synthetic data');
          assert_equals( e.dataTransfer.getData('text'), '', 'step2' );
        });
      };
      t3.step(function() {
   var evt = new DragEvent('dragend', {dataTransfer:e.dataTransfer});
        div.dispatchEvent(evt);
      });
    };
    t3.step(function() {
   var evt = new DragEvent('drag', {dataTransfer:e.dataTransfer});
      blue.dispatchEvent(evt);
    });
    t3.done();
  };

  fuchsia.ondragenter = fuchsia.ondragover = function (e) {
    e.preventDefault();
    if( evtdone[e.type] ) { return; }
    evtdone[e.type] = true;
    var evtype = e.type;

    var t = async_test(e.type+' should share its data with the synthetic event');
    blue.ondragstart = function (e) {
      t.step(function() {
        assert_true( e.dataTransfer.items.length > 0, 'items.length' );
      });
    };
    t.step(function() {
   var evt = new DragEvent('dragstart', {dataTransfer:e.dataTransfer});
      blue.dispatchEvent(evt);
    });
    t.done();

    var t2 = async_test(e.type+' should share its protection status with the synthetic event');
    blue.ondragstart = function (e) {
      t2.step(function() {
        assert_equals( e.dataTransfer.getData('text'), '', 'step 1' );
        e.dataTransfer.setData('text',evtype+'-dragstart synthetic data');
        assert_equals( e.dataTransfer.getData('text'), '', 'step 2' );
      });
    };
    t2.step(function() {
   var evt = new DragEvent('dragstart', {dataTransfer:e.dataTransfer});
      blue.dispatchEvent(evt);
    });
    t2.done();

    test(function() {
      assert_equals( e.dataTransfer.getData('text'), '' );
    }, e.type+' protection status should not be modified by the synthetic event' );

    var t3 = async_test(e.type+' should share its protection status with the nested synthetic event');
    blue.ondragstart = function (e) {
      var div = document.createElement('div');
      div.ondragstart = function (e) {
        t3.step(function() {
          assert_equals( e.dataTransfer.getData('text'), '', 'step1' );
          e.dataTransfer.setData('text',evtype+'dragstart-dragstart synthetic data');
          assert_equals( e.dataTransfer.getData('text'), '', 'step2' );
        });
      };
      t3.step(function() {
    var evt = new DragEvent('dragend', {dataTransfer:e.dataTransfer});
        div.dispatchEvent(evt);
      });
    };
    t3.step(function() {
   var evt = new DragEvent('drag', {dataTransfer:e.dataTransfer});
      blue.dispatchEvent(evt);
    });
    t3.done();
  };

  fuchsia.ondrop = function (e) {
    e.preventDefault();
    if( evtdone[e.type] ) { return; }
    evtdone[e.type] = true;
    var evtype = e.type;

    var t = async_test(e.type+' should share its data with the synthetic event');
    blue.ondragstart = function (e) {
      t.step(function() {
        assert_equals( e.dataTransfer.getData('text'), 'dragstart-drag-dragend synthetic data' );
      });
    };
    t.step(function() {
   var evt = new DragEvent('dragstart', {dataTransfer:e.dataTransfer});
      blue.dispatchEvent(evt);
    });
    t.done();

    var t2 = async_test(e.type+' should share its protection status with the synthetic event');
    blue.ondragstart = function (e) {
      t2.step(function() {
        assert_equals( e.dataTransfer.getData('text'), 'dragstart-drag-dragend synthetic data', 'step 1' );
        e.dataTransfer.setData('text',evtype+'-dragstart synthetic data');
        assert_equals( e.dataTransfer.getData('text'), 'dragstart-drag-dragend synthetic data', 'step 2' );
      });
    };
    t2.step(function() {
   var evt = new DragEvent('dragstart', {dataTransfer:e.dataTransfer});
      blue.dispatchEvent(evt);
    });
    t2.done();

    test(function() {
      e.dataTransfer.setData('text','drop synthetic data');
      assert_equals( e.dataTransfer.getData('text'), 'dragstart-drag-dragend synthetic data' );
    }, e.type+' protection status should not be modified by the synthetic event' );

    var t3 = async_test(e.type+' should share its protection status with the nested synthetic event');
    blue.ondragstart = function (e) {
      var div = document.createElement('div');
      div.ondragstart = function (e) {
        t3.step(function() {
          assert_equals( e.dataTransfer.getData('text'), 'dragstart-drag-dragend synthetic data', 'step 1' );
          e.dataTransfer.setData('text',evtype+'dragstart-dragstart synthetic data');
          assert_equals( e.dataTransfer.getData('text'), 'dragstart-drag-dragend synthetic data', 'step 2' );
        });
      };
      t3.step(function() {
   var evt = new DragEvent('dragend', {dataTransfer:e.dataTransfer});
        div.dispatchEvent(evt);
      });
    };
    t3.step(function() {
   var evt = new DragEvent('drag', {dataTransfer:e.dataTransfer});
      blue.dispatchEvent(evt);
    });
    t3.done();
  };

  orange.ondragend = function (e) {
    if( evtdone[e.type] ) { return; }
    evtdone[e.type] = true;
    var evtype = e.type;

    var t = async_test(e.type+' should share its data with the synthetic event');
    blue.ondragstart = function (e) {
      t.step(function() {
        assert_true( e.dataTransfer.items.length > 0, 'items.length' );
      });
    };
    t.step(function() {
   var evt = new DragEvent('dragstart', {dataTransfer:e.dataTransfer});
      blue.dispatchEvent(evt);
    });
    t.done();

    var t2 = async_test(e.type+' should share its protection status with the synthetic event');
    blue.ondragstart = function (e) {
      t2.step(function() {
        assert_equals( e.dataTransfer.getData('text'), '', 'step 1' );
        e.dataTransfer.setData('text',evtype+'-dragstart synthetic data');
        assert_equals( e.dataTransfer.getData('text'), '', 'step 2' );
      });
    };
    t2.step(function() {
   var evt = new DragEvent('dragstart', {dataTransfer:e.dataTransfer});
      blue.dispatchEvent(evt);
    });
    t2.done();

    test(function() {
      assert_equals( e.dataTransfer.getData('text'), '' );
    }, e.type+' protection status should not be modified by the synthetic event' );

    var t3 = async_test(e.type+' should share its protection status with the nested synthetic event');
    blue.ondragstart = function (e) {
      var div = document.createElement('div');
      div.ondragstart = function (e) {
        t3.step(function() {
          assert_equals( e.dataTransfer.getData('text'), '', 'step1' );
          e.dataTransfer.setData('text',evtype+'dragstart-dragstart synthetic data');
          assert_equals( e.dataTransfer.getData('text'), '', 'step2' );
        });
      };
      t3.step(function() {
   var evt = new DragEvent('dragend', {dataTransfer:e.dataTransfer});
        div.dispatchEvent(evt);
      });
    };
    t3.step(function() {
   var evt = new DragEvent('drag', {dataTransfer:e.dataTransfer});
      blue.dispatchEvent(evt);
    });
    t3.done();

    test(function() {
      var failtxt = '- Reload and try again';
      assert_true( evtdone.dragstart, 'dragstart event was not tested'+failtxt );
      assert_true( evtdone.drag, 'drag event was not tested'+failtxt );
      assert_true( evtdone.dragenter, 'dragenter event was not tested'+failtxt );
      assert_true( evtdone.dragleave, 'dragleave event was not tested'+failtxt );
      assert_true( evtdone.dragover, 'dragover event was not tested'+failtxt );
      assert_true( evtdone.drop, 'drop event was not tested'+failtxt );
      assert_true( evtdone.dragend, 'dragend event was not tested'+failtxt );
    }, 'all event types must now have been tested' );
    done();
  };

};
    </script>
  </head>
  <body>
    <p>Drag the orange square over the blue square then the fuchsia square, then release it.</p>
    <blockquote draggable="true"></blockquote>
    <blockquote></blockquote>
    <blockquote></blockquote>
    <div id="log"></div>
    <noscript><p>Enable JavaScript and reload</p></noscript>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 164,
        "byte_start": 141,
        "col": 5,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 487,
        "byte_start": 424,
        "col": 5,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 570,
        "byte_start": 501,
        "col": 5,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 615,
        "byte_start": 584,
        "col": 5,
        "line": 13
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
  "source_name": "html/editing/dnd/synthetic/005-manual.html"
}
```
