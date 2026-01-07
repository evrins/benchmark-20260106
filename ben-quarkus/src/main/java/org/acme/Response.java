package org.acme;

import java.util.HashMap;

public class Response {
    private String msg = "OK";
    private int code = 0;
    private HashMap<String, Object> data = new HashMap<>();

    public String getMsg() {
        return msg;
    }

    public int getCode() {
        return code;
    }

    public Object getData() {
        return data;
    }
}