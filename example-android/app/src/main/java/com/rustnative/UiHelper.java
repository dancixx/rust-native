package com.rustnative;

import android.app.Activity;
import android.view.View;

public class UiHelper {

    public static void setContentViewOnUiThread(
        Activity activity,
        View rootView
    ) {
        activity.runOnUiThread(() -> activity.setContentView(rootView));
    }
}
