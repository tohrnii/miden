use super::build_test;

#[test]
fn test_poly512_mul_zq() {
    let source = "
    use.std::math::poly512

    proc.wrapper.384
        # prepare first polynomial `f`

        push.18446744069414584303.128.23.18446744069414584303
        loc_storew.127
        dropw
        
        push.18446744069414584210.18446744069414584264.226.155
        loc_storew.126
        dropw
        
        push.101.18446744069414584266.18446744069414584135.18446744069414584248
        loc_storew.125
        dropw
        
        push.18446744069414584187.348.15.332
        loc_storew.124
        dropw
        
        push.18446744069414584036.231.18446744069414584220.18446744069414584273
        loc_storew.123
        dropw
        
        push.155.63.18446744069414584189.226
        loc_storew.122
        dropw

        
        push.18446744069414584292.18446744069414584216.18446744069414583997.240
        loc_storew.121
        dropw

        
        push.18446744069414583940.135.18446744069414584286.18446744069414584093
        loc_storew.120
        dropw

        
        push.106.24.185.18446744069414584133
        loc_storew.119
        dropw

        
        push.18446744069414584198.18446744069414584022.18446744069414584303.18446744069414584152
        loc_storew.118
        dropw

        
        push.18446744069414584301.183.38.18446744069414584233
        loc_storew.117
        dropw

        
        push.18446744069414584287.299.18446744069414584221.18446744069414584312
        loc_storew.116
        dropw

        
        push.168.18446744069414584148.16.18446744069414584173
        loc_storew.115
        dropw

        
        push.18446744069414584307.18446744069414584204.403.18446744069414584172
        loc_storew.114
        dropw

        
        push.18446744069414584195.18446744069414584215.18446744069414584318.30
        loc_storew.113
        dropw

        
        push.304.18446744069414584183.18446744069414584249.18446744069414584284
        loc_storew.112
        dropw

        
        push.18446744069414584309.51.125.103
        loc_storew.111
        dropw

        
        push.18446744069414584286.18446744069414584269.76.18446744069414584293
        loc_storew.110
        dropw

        
        push.18446744069414584123.203.271.55
        loc_storew.109
        dropw

        
        push.65.99.125.230
        loc_storew.108
        dropw

        
        push.150.48.226.41
        loc_storew.107
        dropw

        
        push.184.18446744069414584163.18446744069414584295.8
        loc_storew.106
        dropw

        
        push.159.18446744069414584242.127.18446744069414584291
        loc_storew.105
        dropw

        
        push.167.18446744069414584228.18446744069414584287.18446744069414584255
        loc_storew.104
        dropw

        
        push.144.18446744069414584264.18446744069414584268.51
        loc_storew.103
        dropw

        
        push.78.205.408.18446744069414584271
        loc_storew.102
        dropw

        
        push.18446744069414584077.58.196.18446744069414584273
        loc_storew.101
        dropw

        
        push.18446744069414584057.18446744069414584128.31.18446744069414584255
        loc_storew.100
        dropw

        
        push.22.18446744069414584148.18446744069414584217.136
        loc_storew.99
        dropw
        
        push.18446744069414584119.18446744069414584240.18446744069414584288.18446744069414584031
        loc_storew.98
        dropw
        
        push.18446744069414584164.221.73.18446744069414584285
        loc_storew.97
        dropw
        
        push.18446744069414584220.18446744069414584202.183.18446744069414584226
        loc_storew.96
        dropw
        
        push.18446744069414584274.124.4.18446744069414584292
        loc_storew.95
        dropw
        
        push.5.18446744069414583953.20.25
        loc_storew.94
        dropw
        
        push.209.242.86.18446744069414584275
        loc_storew.93
        dropw
        
        push.176.18446744069414584315.131.54
        loc_storew.92
        dropw
        
        push.8.135.177.18446744069414584142
        loc_storew.91
        dropw
        
        push.108.66.178.331
        loc_storew.90
        dropw
        
        push.18446744069414584224.6.170.110
        loc_storew.89
        dropw
        
        push.67.18446744069414584124.18446744069414584134.18446744069414584012
        loc_storew.88
        dropw
        
        push.118.228.23.18446744069414584319
        loc_storew.87
        dropw
        
        push.25.18446744069414583961.160.18446744069414584181
        loc_storew.86
        dropw
        
        push.18446744069414584202.18446744069414584212.18446744069414584246.18446744069414584141
        loc_storew.85
        dropw
        
        push.0.18446744069414584164.18446744069414584158.19
        loc_storew.84
        dropw
        
        push.18446744069414583944.48.41.18446744069414584177
        loc_storew.83
        dropw
        
        push.18446744069414583980.22.18446744069414584160.234
        loc_storew.82
        dropw
        
        push.18446744069414584119.18446744069414584091.18446744069414584116.58
        loc_storew.81
        dropw
        
        push.18446744069414584315.16.18446744069414583851.8
        loc_storew.80
        dropw
        
        push.18446744069414584027.18446744069414584229.18446744069414584254.49
        loc_storew.79
        dropw
        
        push.18446744069414584278.6.34.107
        loc_storew.78
        dropw
        
        push.18446744069414584153.89.18446744069414584012.21
        loc_storew.77
        dropw
        
        push.18446744069414584164.18446744069414584004.337.18446744069414584315
        loc_storew.76
        dropw
        
        push.110.106.43.18446744069414584175
        loc_storew.75
        dropw
        
        push.18446744069414584262.15.105.98
        loc_storew.74
        dropw
        
        push.81.30.105.196
        loc_storew.73
        dropw
        
        push.18446744069414584021.18446744069414584315.18446744069414584274.18446744069414584310
        loc_storew.72
        dropw
        
        push.18446744069414584295.177.2.167
        loc_storew.71
        dropw
        
        push.195.18446744069414584163.124.18446744069414583936
        loc_storew.70
        dropw
        
        push.67.247.18446744069414583995.18446744069414584134
        loc_storew.69
        dropw
        
        push.35.234.211.18446744069414584211
        loc_storew.68
        dropw
        
        push.18446744069414584274.18446744069414583923.59.18446744069414584314
        loc_storew.67
        dropw
        
        push.18446744069414584289.45.18446744069414584139.18446744069414584280
        loc_storew.66
        dropw
        
        push.8.72.13.88
        loc_storew.65
        dropw
        
        push.69.141.18446744069414584226.18446744069414584199
        loc_storew.64
        dropw
        
        push.15.82.18446744069414584287.18446744069414584208
        loc_storew.63
        dropw
        
        push.200.18446744069414584288.18446744069414584065.18446744069414584296
        loc_storew.62
        dropw
        
        push.18446744069414584170.374.18446744069414584048.82
        loc_storew.61
        dropw
        
        push.293.175.158.18446744069414584250
        loc_storew.60
        dropw
        
        push.20.18446744069414584280.18446744069414584191.100
        loc_storew.59
        dropw
        
        push.138.18446744069414584231.18446744069414584044.18446744069414584270
        loc_storew.58
        dropw
        
        push.18446744069414584179.18446744069414584228.18446744069414584289.139
        loc_storew.57
        dropw
        
        push.18446744069414584133.18446744069414584291.334.76
        loc_storew.56
        dropw
        
        push.18446744069414584242.18446744069414584172.18446744069414584222.18446744069414584297
        loc_storew.55
        dropw
        
        push.127.291.100.18446744069414584151
        loc_storew.54
        dropw
        
        push.51.149.389.18446744069414584136
        loc_storew.53
        dropw
        
        push.18446744069414584199.243.18446744069414584130.18446744069414583899
        loc_storew.52
        dropw
        
        push.273.18446744069414584306.18446744069414584114.185
        loc_storew.51
        dropw
        
        push.18446744069414583974.18446744069414584302.2.67
        loc_storew.50
        dropw
        
        push.403.9.18446744069414584210.298
        loc_storew.49
        dropw
        
        push.18446744069414584271.18446744069414584102.18446744069414584203.18446744069414584237
        loc_storew.48
        dropw
        
        push.19.200.121.132
        loc_storew.47
        dropw
        
        push.18446744069414584022.18446744069414584234.18446744069414584143.93
        loc_storew.46
        dropw
        
        push.18446744069414584234.484.18446744069414584271.18446744069414584120
        loc_storew.45
        dropw
        
        push.18446744069414584244.114.4.88
        loc_storew.44
        dropw
        
        push.27.18446744069414584035.18446744069414584120.18446744069414584019
        loc_storew.43
        dropw
        
        push.37.18446744069414584266.18446744069414584161.3
        loc_storew.42
        dropw
        
        push.18446744069414584167.18446744069414584206.212.18446744069414584175
        loc_storew.41
        dropw
        
        push.18446744069414584279.25.18446744069414584245.4
        loc_storew.40
        dropw
        
        push.18446744069414584049.73.117.4
        loc_storew.39
        dropw
        
        push.180.18446744069414584078.61.18446744069414584287
        loc_storew.38
        dropw
        
        push.273.18446744069414584320.18446744069414584168.18446744069414584178
        loc_storew.37
        dropw
        
        push.18446744069414584284.18446744069414584200.18446744069414584134.18446744069414584290
        loc_storew.36
        dropw
        
        push.182.41.18446744069414584094.18446744069414584174
        loc_storew.35
        dropw
        
        push.18446744069414584275.59.306.18446744069414584295
        loc_storew.34
        dropw
        
        push.80.238.18446744069414584224.61
        loc_storew.33
        dropw
        
        push.18446744069414584305.18446744069414584086.18446744069414583951.149
        loc_storew.32
        dropw
        
        push.18446744069414584271.18446744069414584144.18446744069414584080.18446744069414584265
        loc_storew.31
        dropw
        
        push.18446744069414584315.77.18446744069414584287.152
        loc_storew.30
        dropw
        
        push.118.16.18446744069414584073.18446744069414584190
        loc_storew.29
        dropw
        
        push.42.18446744069414584251.187.170
        loc_storew.28
        dropw
        
        push.18446744069414583929.18446744069414584276.18446744069414584214.18446744069414584279
        loc_storew.27
        dropw
        
        push.14.18446744069414584262.18446744069414584192.162
        loc_storew.26
        dropw
        
        push.18446744069414584253.17.18446744069414584298.18446744069414584117
        loc_storew.25
        dropw
        
        push.168.60.85.18446744069414584291
        loc_storew.24
        dropw
        
        push.18446744069414584151.18446744069414584282.18446744069414584231.72
        loc_storew.23
        dropw
        
        push.18446744069414584292.207.33.18446744069414584246
        loc_storew.22
        dropw
        
        push.53.100.53.56
        loc_storew.21
        dropw
        
        push.18446744069414584249.122.18446744069414584091.18446744069414584286
        loc_storew.20
        dropw
        
        push.18446744069414584061.18446744069414584261.18446744069414584241.0
        loc_storew.19
        dropw
        
        push.18446744069414584186.82.92.47
        loc_storew.18
        dropw
        
        push.8.18446744069414584155.18446744069414584174.18446744069414584209
        loc_storew.17
        dropw
        
        push.18446744069414584214.164.378.18446744069414584300
        loc_storew.16
        dropw
        
        push.18446744069414584140.18446744069414584170.18446744069414584173.18446744069414584221
        loc_storew.15
        dropw
        
        push.45.189.331.18446744069414584104
        loc_storew.14
        dropw
        
        push.18446744069414584275.133.18446744069414584196.18446744069414584239
        loc_storew.13
        dropw
        
        push.128.160.18446744069414584044.52
        loc_storew.12
        dropw
        
        push.8.97.18446744069414584047.14
        loc_storew.11
        dropw
        
        push.18446744069414584173.18446744069414584178.130.18446744069414584204
        loc_storew.10
        dropw
        
        push.34.18446744069414583997.103.20
        loc_storew.9
        dropw
        
        push.18446744069414584192.18446744069414584126.18.18446744069414584151
        loc_storew.8
        dropw
        
        push.18446744069414584299.39.18446744069414584160.302
        loc_storew.7
        dropw
        
        push.32.184.18446744069414584288.18446744069414584126
        loc_storew.6
        dropw
        
        push.283.18446744069414584308.18446744069414584287.18446744069414584288
        loc_storew.5
        dropw
        
        push.43.18446744069414584073.121.18446744069414584082
        loc_storew.4
        dropw
        
        push.18446744069414584294.18446744069414584154.18446744069414584111.100
        loc_storew.3
        dropw
        
        push.246.241.18446744069414584043.18446744069414584117
        loc_storew.2
        dropw
        
        push.18446744069414584114.144.18446744069414584301.18446744069414584118
        loc_storew.1
        dropw
        
        push.18446744069414584294.132.18446744069414584309.80
        loc_storew.0
        dropw

        # prepare second polynomial `g`

        push.8513.6367.8750.11496
        loc_storew.255
        dropw


        push.7720.11184.2801.9698
        loc_storew.254
        dropw


        push.6495.12169.6551.3044
        loc_storew.253
        dropw


        push.2608.3965.10601.2608
        loc_storew.252
        dropw


        push.11190.5015.5266.6931
        loc_storew.251
        dropw


        push.6906.2735.11241.11904
        loc_storew.250
        dropw


        push.9359.4500.6600.7831
        loc_storew.249
        dropw


        push.2589.8774.5436.4245
        loc_storew.248
        dropw


        push.8332.696.8983.4561
        loc_storew.247
        dropw


        push.7575.2855.1996.4550
        loc_storew.246
        dropw


        push.12283.869.2784.2429
        loc_storew.245
        dropw


        push.2406.8000.11327.7148
        loc_storew.244
        dropw


        push.10658.9693.7003.9422
        loc_storew.243
        dropw


        push.1465.240.7617.1286
        loc_storew.242
        dropw


        push.10912.6893.9727.4821
        loc_storew.241
        dropw


        push.5020.11575.10947.4320
        loc_storew.240
        dropw


        push.982.12228.9103.1246
        loc_storew.239
        dropw


        push.1984.5066.5442.1652
        loc_storew.238
        dropw


        push.6828.11600.10958.5969
        loc_storew.237
        dropw


        push.8427.11562.9074.10785
        loc_storew.236
        dropw


        push.9884.3146.10225.7384
        loc_storew.235
        dropw


        push.7012.6914.10528.227
        loc_storew.234
        dropw


        push.2442.2344.618.11418
        loc_storew.233
        dropw


        push.9.4659.1590.12118
        loc_storew.232
        dropw


        push.7889.1062.2974.6054
        loc_storew.231
        dropw


        push.3953.10955.11552.7428
        loc_storew.230
        dropw


        push.6419.3360.5488.11650
        loc_storew.229
        dropw


        push.10273.11937.7855.2018
        loc_storew.228
        dropw


        push.9827.2946.10619.11760
        loc_storew.227
        dropw


        push.7879.10081.5288.1391
        loc_storew.226
        dropw


        push.4719.10976.2821.436
        loc_storew.225
        dropw


        push.2921.9630.9319.3805
        loc_storew.224
        dropw


        push.822.8476.11006.4919
        loc_storew.223
        dropw


        push.2966.3539.6488.3362
        loc_storew.222
        dropw


        push.6766.3581.11199.9066
        loc_storew.221
        dropw


        push.1904.8230.5432.9874
        loc_storew.220
        dropw


        push.3017.650.9536.10886
        loc_storew.219
        dropw


        push.10043.11999.3273.8013
        loc_storew.218
        dropw


        push.9709.3001.8661.9288
        loc_storew.217
        dropw


        push.5174.3436.7455.1944
        loc_storew.216
        dropw


        push.10546.7710.5047.887
        loc_storew.215
        dropw


        push.6055.10870.11586.5349
        loc_storew.214
        dropw


        push.7852.2913.5456.587
        loc_storew.213
        dropw


        push.6656.11242.89.4569
        loc_storew.212
        dropw


        push.1074.11556.5474.7772
        loc_storew.211
        dropw


        push.11848.6103.8253.5017
        loc_storew.210
        dropw


        push.5651.4405.6126.4716
        loc_storew.209
        dropw


        push.7603.11740.369.6845
        loc_storew.208
        dropw


        push.6450.915.7584.7746
        loc_storew.207
        dropw


        push.9124.256.10494.9542
        loc_storew.206
        dropw


        push.1531.7618.8698.4106
        loc_storew.205
        dropw


        push.1120.1711.9513.11543
        loc_storew.204
        dropw


        push.7814.947.11319.6401
        loc_storew.203
        dropw


        push.1379.10521.7342.4649
        loc_storew.202
        dropw


        push.6221.6053.4336.7114
        loc_storew.201
        dropw


        push.10946.8195.3752.1914
        loc_storew.200
        dropw


        push.6416.11370.1259.5208
        loc_storew.199
        dropw


        push.7596.8682.5381.5131
        loc_storew.198
        dropw


        push.11788.11339.2484.8281
        loc_storew.197
        dropw


        push.6449.2273.5553.7058
        loc_storew.196
        dropw


        push.2901.4196.11847.608
        loc_storew.195
        dropw


        push.9934.3256.6603.12045
        loc_storew.194
        dropw


        push.907.11513.8114.7986
        loc_storew.193
        dropw


        push.4038.4668.6623.8637
        loc_storew.192
        dropw


        push.6388.4283.5537.11237
        loc_storew.191
        dropw


        push.2128.2128.8930.6134
        loc_storew.190
        dropw


        push.7762.8973.7004.2963
        loc_storew.189
        dropw


        push.745.7196.10591.171
        loc_storew.188
        dropw


        push.8891.10421.2633.2586
        loc_storew.187
        dropw


        push.4723.2007.4224.3400
        loc_storew.186
        dropw


        push.722.8976.2104.10362
        loc_storew.185
        dropw


        push.6241.6325.2652.11441
        loc_storew.184
        dropw


        push.9040.7855.11748.2988
        loc_storew.183
        dropw


        push.867.9770.9407.7088
        loc_storew.182
        dropw


        push.1082.12110.4362.2077
        loc_storew.181
        dropw


        push.10985.4330.4862.1850
        loc_storew.180
        dropw


        push.2619.7677.10483.5379
        loc_storew.179
        dropw


        push.6398.2103.3252.2355
        loc_storew.178
        dropw


        push.9556.3245.3782.11488
        loc_storew.177
        dropw


        push.8587.8334.4738.5907
        loc_storew.176
        dropw


        push.8498.6495.5343.6139
        loc_storew.175
        dropw


        push.10159.8532.10335.7104
        loc_storew.174
        dropw


        push.12269.10616.9264.8308
        loc_storew.173
        dropw


        push.1508.4838.1430.4354
        loc_storew.172
        dropw


        push.11497.6956.2651.10559
        loc_storew.171
        dropw


        push.4011.2791.1131.8752
        loc_storew.170
        dropw


        push.5714.9498.3438.4253
        loc_storew.169
        dropw


        push.5019.5480.10070.10445
        loc_storew.168
        dropw


        push.3066.1261.7725.6473
        loc_storew.167
        dropw


        push.3496.2246.7815.198
        loc_storew.166
        dropw


        push.5569.5866.739.8064
        loc_storew.165
        dropw


        push.8395.668.2244.11456
        loc_storew.164
        dropw


        push.9293.4408.2772.5445
        loc_storew.163
        dropw


        push.11571.3718.761.11014
        loc_storew.162
        dropw


        push.10321.3579.368.3404
        loc_storew.161
        dropw


        push.529.10187.11875.6736
        loc_storew.160
        dropw


        push.4932.2568.2368.280
        loc_storew.159
        dropw


        push.7205.7792.7260.6205
        loc_storew.158
        dropw


        push.3502.11963.1381.11919
        loc_storew.157
        dropw


        push.4892.9950.7457.11363
        loc_storew.156
        dropw


        push.711.10007.5957.10373
        loc_storew.155
        dropw


        push.8934.8529.2571.11549
        loc_storew.154
        dropw


        push.5302.6209.4109.5748
        loc_storew.153
        dropw


        push.7545.3825.1970.5566
        loc_storew.152
        dropw


        push.2503.7545.11519.351
        loc_storew.151
        dropw


        push.4183.2813.1449.3567
        loc_storew.150
        dropw


        push.8500.6684.12054.7617
        loc_storew.149
        dropw


        push.10069.4403.2228.1397
        loc_storew.148
        dropw


        push.1364.9204.4417.7801
        loc_storew.147
        dropw


        push.9585.8282.3708.3084
        loc_storew.146
        dropw


        push.6005.4234.10093.5338
        loc_storew.145
        dropw


        push.5204.3841.1525.8209
        loc_storew.144
        dropw


        push.8948.3108.2267.2613
        loc_storew.143
        dropw


        push.9187.7324.7531.8153
        loc_storew.142
        dropw


        push.5060.4422.684.2570
        loc_storew.141
        dropw


        push.707.3214.11619.8768
        loc_storew.140
        dropw


        push.4774.169.5379.7175
        loc_storew.139
        dropw


        push.11514.3021.6510.6508
        loc_storew.138
        dropw


        push.3453.3931.4509.179
        loc_storew.137
        dropw


        push.12029.4043.4992.7772
        loc_storew.136
        dropw


        push.5730.8752.9766.8039
        loc_storew.135
        dropw


        push.9754.8370.2055.5298
        loc_storew.134
        dropw


        push.2970.9288.731.2872
        loc_storew.133
        dropw


        push.4920.10632.5281.315
        loc_storew.132
        dropw


        push.3040.4981.5117.609
        loc_storew.131
        dropw


        push.10176.695.1530.9677
        loc_storew.130
        dropw


        push.6452.2120.3336.5260
        loc_storew.129
        dropw


        push.4868.5640.3911.6772
        loc_storew.128
        dropw


        # prepare argument ( absolute memory addresses ) for multiplying two polynomials

        locaddr.383 # output
        locaddr.255 # input 1
        locaddr.127 # input 0

        # perform polynomial multiplication, when two polynomials are provided
        # as absolute memory addresses on the stack

        exec.poly512::mul_zq

        # check for functional correctness ( using known answer test )

        locaddr.383

        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.4273
        assert_eq
        push.7261
        assert_eq
        push.7665
        assert_eq
        push.7741
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.2447
        assert_eq
        push.7581
        assert_eq
        push.8745
        assert_eq
        push.222
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.9716
        assert_eq
        push.5487
        assert_eq
        push.516
        assert_eq
        push.338
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6927
        assert_eq
        push.6586
        assert_eq
        push.285
        assert_eq
        push.11359
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.8285
        assert_eq
        push.4065
        assert_eq
        push.11968
        assert_eq
        push.10394
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.2139
        assert_eq
        push.1450
        assert_eq
        push.2112
        assert_eq
        push.5902
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.5205
        assert_eq
        push.10169
        assert_eq
        push.12222
        assert_eq
        push.10743
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.10052
        assert_eq
        push.4937
        assert_eq
        push.5329
        assert_eq
        push.1081
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.7372
        assert_eq
        push.9075
        assert_eq
        push.2675
        assert_eq
        push.10821
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6781
        assert_eq
        push.12247
        assert_eq
        push.396
        assert_eq
        push.8559
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.7955
        assert_eq
        push.7749
        assert_eq
        push.1625
        assert_eq
        push.1761
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.1346
        assert_eq
        push.756
        assert_eq
        push.5927
        assert_eq
        push.2250
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6472
        assert_eq
        push.5565
        assert_eq
        push.6005
        assert_eq
        push.6231
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.5228
        assert_eq
        push.401
        assert_eq
        push.2170
        assert_eq
        push.8943
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6071
        assert_eq
        push.6011
        assert_eq
        push.11840
        assert_eq
        push.10128
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.8101
        assert_eq
        push.11721
        assert_eq
        push.2877
        assert_eq
        push.3548
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.4724
        assert_eq
        push.3816
        assert_eq
        push.9576
        assert_eq
        push.6914
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.11614
        assert_eq
        push.3083
        assert_eq
        push.315
        assert_eq
        push.267
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.3547
        assert_eq
        push.1881
        assert_eq
        push.3496
        assert_eq
        push.9629
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.2660
        assert_eq
        push.10403
        assert_eq
        push.980
        assert_eq
        push.10655
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.4179
        assert_eq
        push.11661
        assert_eq
        push.9598
        assert_eq
        push.11119
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.9834
        assert_eq
        push.8521
        assert_eq
        push.7064
        assert_eq
        push.5155
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.11311
        assert_eq
        push.3859
        assert_eq
        push.1707
        assert_eq
        push.8651
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.2426
        assert_eq
        push.596
        assert_eq
        push.7821
        assert_eq
        push.12130
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.9806
        assert_eq
        push.660
        assert_eq
        push.3518
        assert_eq
        push.4797
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.9000
        assert_eq
        push.11879
        assert_eq
        push.5421
        assert_eq
        push.9044
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.917
        assert_eq
        push.4404
        assert_eq
        push.7444
        assert_eq
        push.2878
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.10618
        assert_eq
        push.2011
        assert_eq
        push.6703
        assert_eq
        push.777
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.9360
        assert_eq
        push.6051
        assert_eq
        push.4333
        assert_eq
        push.4915
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.11693
        assert_eq
        push.11914
        assert_eq
        push.325
        assert_eq
        push.11143
        assert_eq
        add.1

        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6590
        assert_eq
        push.5012
        assert_eq
        push.6053
        assert_eq
        push.7200
        assert_eq
        add.1

        dup    
        push.0.0.0.0
        movup.4
        mem_loadw
        push.3799
        assert_eq
        push.1985
        assert_eq
        push.450
        assert_eq
        push.2956
        assert_eq
        add.1

        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.1028
        assert_eq
        push.6658
        assert_eq
        push.4582
        assert_eq
        push.2286
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6752
        assert_eq
        push.165
        assert_eq
        push.3006
        assert_eq
        push.875
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.7575
        assert_eq
        push.2134
        assert_eq
        push.628
        assert_eq
        push.3081
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.12287
        assert_eq
        push.11809
        assert_eq
        push.1644
        assert_eq
        push.460
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6547
        assert_eq
        push.9171
        assert_eq
        push.8751
        assert_eq
        push.9948
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.3241
        assert_eq
        push.5957
        assert_eq
        push.11149
        assert_eq
        push.338
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.7752
        assert_eq
        push.9000
        assert_eq
        push.11899
        assert_eq
        push.572
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6166
        assert_eq
        push.7751
        assert_eq
        push.11228
        assert_eq
        push.2817
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.8570
        assert_eq
        push.4532
        assert_eq
        push.9839
        assert_eq
        push.179
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.11062
        assert_eq
        push.996
        assert_eq
        push.5462
        assert_eq
        push.4512
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6848
        assert_eq
        push.11427
        assert_eq
        push.2327
        assert_eq
        push.6122
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.4612
        assert_eq
        push.11240
        assert_eq
        push.11153
        assert_eq
        push.5857
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.3207
        assert_eq
        push.11725
        assert_eq
        push.6301
        assert_eq
        push.2132
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.7461
        assert_eq
        push.7255
        assert_eq
        push.3533
        assert_eq
        push.3202
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.3873
        assert_eq
        push.3119
        assert_eq
        push.935
        assert_eq
        push.2439
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6826
        assert_eq
        push.1220
        assert_eq
        push.9335
        assert_eq
        push.4582
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6425
        assert_eq
        push.10466
        assert_eq
        push.11338
        assert_eq
        push.9401
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.389
        assert_eq
        push.1250
        assert_eq
        push.1649
        assert_eq
        push.8194
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.5199
        assert_eq
        push.11095
        assert_eq
        push.8865
        assert_eq
        push.262
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.8101
        assert_eq
        push.8533
        assert_eq
        push.8700
        assert_eq
        push.492
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.8104
        assert_eq
        push.11544
        assert_eq
        push.9703
        assert_eq
        push.3300
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.2987
        assert_eq
        push.7458
        assert_eq
        push.8853
        assert_eq
        push.5387
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6423
        assert_eq
        push.2601
        assert_eq
        push.6928
        assert_eq
        push.11947
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.4817
        assert_eq
        push.1786
        assert_eq
        push.3786
        assert_eq
        push.3629
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.11214
        assert_eq
        push.4449
        assert_eq
        push.3446
        assert_eq
        push.5536
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.3020
        assert_eq
        push.8984
        assert_eq
        push.2095
        assert_eq
        push.5141
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.5348
        assert_eq
        push.6591
        assert_eq
        push.2331
        assert_eq
        push.11361
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.8652
        assert_eq
        push.11887
        assert_eq
        push.8313
        assert_eq
        push.11891
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.5291
        assert_eq
        push.4281
        assert_eq
        push.8371
        assert_eq
        push.1956
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.7931
        assert_eq
        push.1159
        assert_eq
        push.3320
        assert_eq
        push.6645
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.3675
        assert_eq
        push.10557
        assert_eq
        push.2558
        assert_eq
        push.3758
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.7653
        assert_eq
        push.1638
        assert_eq
        push.8160
        assert_eq
        push.10651
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.8007
        assert_eq
        push.5593
        assert_eq
        push.6552
        assert_eq
        push.5805
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.10589
        assert_eq
        push.857
        assert_eq
        push.805
        assert_eq
        push.2090
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.5515
        assert_eq
        push.3098
        assert_eq
        push.10230
        assert_eq
        push.9292
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.476
        assert_eq
        push.458
        assert_eq
        push.11242
        assert_eq
        push.12027
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.12002
        assert_eq
        push.4691
        assert_eq
        push.5208
        assert_eq
        push.4893
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.3674
        assert_eq
        push.6957
        assert_eq
        push.11574
        assert_eq
        push.4570
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6559
        assert_eq
        push.4612
        assert_eq
        push.10451
        assert_eq
        push.3813
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.5224
        assert_eq
        push.6824
        assert_eq
        push.2777
        assert_eq
        push.525
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.10411
        assert_eq
        push.4425
        assert_eq
        push.8664
        assert_eq
        push.3033
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.10736
        assert_eq
        push.9042
        assert_eq
        push.9737
        assert_eq
        push.983
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.5733
        assert_eq
        push.9958
        assert_eq
        push.5959
        assert_eq
        push.8180
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.2150
        assert_eq
        push.9845
        assert_eq
        push.8923
        assert_eq
        push.2445
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.10855
        assert_eq
        push.12268
        assert_eq
        push.1154
        assert_eq
        push.7385
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.7955
        assert_eq
        push.5279
        assert_eq
        push.7945
        assert_eq
        push.4142
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.361
        assert_eq
        push.10477
        assert_eq
        push.10540
        assert_eq
        push.6464
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.753
        assert_eq
        push.10998
        assert_eq
        push.1524
        assert_eq
        push.5295
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.8158
        assert_eq
        push.7888
        assert_eq
        push.2866
        assert_eq
        push.11472
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6273
        assert_eq
        push.4479
        assert_eq
        push.9994
        assert_eq
        push.3871
        assert_eq
        add.1 
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.1385
        assert_eq
        push.6087
        assert_eq
        push.9199
        assert_eq
        push.6329
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.8249
        assert_eq
        push.6464
        assert_eq
        push.11072
        assert_eq
        push.2173
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.9758
        assert_eq
        push.10806
        assert_eq
        push.11994
        assert_eq
        push.4747
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.1595
        assert_eq
        push.8112
        assert_eq
        push.5517
        assert_eq
        push.10870
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.7465
        assert_eq
        push.9828
        assert_eq
        push.8595
        assert_eq
        push.11219
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.3180
        assert_eq
        push.6556
        assert_eq
        push.3738
        assert_eq
        push.2276
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6333
        assert_eq
        push.11928
        assert_eq
        push.3480
        assert_eq
        push.11734
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.8004
        assert_eq
        push.3853
        assert_eq
        push.10375
        assert_eq
        push.9461
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.270
        assert_eq
        push.11823
        assert_eq
        push.2252
        assert_eq
        push.2119
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.8782
        assert_eq
        push.10642
        assert_eq
        push.11329
        assert_eq
        push.8675
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.3722
        assert_eq
        push.3657
        assert_eq
        push.6708
        assert_eq
        push.6307
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.9757
        assert_eq
        push.11020
        assert_eq
        push.4138
        assert_eq
        push.4956
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6737
        assert_eq
        push.11644
        assert_eq
        push.5377
        assert_eq
        push.6339
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.7792
        assert_eq
        push.10029
        assert_eq
        push.7725
        assert_eq
        push.3354
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.3477
        assert_eq
        push.10305
        assert_eq
        push.10899
        assert_eq
        push.11773
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.8545
        assert_eq
        push.4005
        assert_eq
        push.8967
        assert_eq
        push.1365
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.12028
        assert_eq
        push.10647
        assert_eq
        push.1919
        assert_eq
        push.9758
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.2857
        assert_eq
        push.5966
        assert_eq
        push.9785
        assert_eq
        push.7332
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.1853
        assert_eq
        push.10441
        assert_eq
        push.5140
        assert_eq
        push.9234
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.4371
        assert_eq
        push.1530
        assert_eq
        push.12180
        assert_eq
        push.99
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.808
        assert_eq
        push.4077
        assert_eq
        push.3847
        assert_eq
        push.4540
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6593
        assert_eq
        push.9915
        assert_eq
        push.6405
        assert_eq
        push.10922
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.16
        assert_eq
        push.8772
        assert_eq
        push.5121
        assert_eq
        push.2671
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.12042
        assert_eq
        push.7102
        assert_eq
        push.4884
        assert_eq
        push.12110
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.5017
        assert_eq
        push.4085
        assert_eq
        push.8820
        assert_eq
        push.9506
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.475
        assert_eq
        push.2368
        assert_eq
        push.9536
        assert_eq
        push.9043
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.1653
        assert_eq
        push.895
        assert_eq
        push.10261
        assert_eq
        push.4781
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.9783
        assert_eq
        push.7521
        assert_eq
        push.9985
        assert_eq
        push.7227
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.8390
        assert_eq
        push.5485
        assert_eq
        push.7840
        assert_eq
        push.2145
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6870
        assert_eq
        push.4260
        assert_eq
        push.9900
        assert_eq
        push.11479
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.38
        assert_eq
        push.11532
        assert_eq
        push.9884
        assert_eq
        push.10573
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.9452
        assert_eq
        push.137
        assert_eq
        push.4261
        assert_eq
        push.4234
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6257
        assert_eq
        push.8225
        assert_eq
        push.12158
        assert_eq
        push.511
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.4020
        assert_eq
        push.6540
        assert_eq
        push.6205
        assert_eq
        push.3269
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.6994
        assert_eq
        push.3077
        assert_eq
        push.2887
        assert_eq
        push.10207
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.9773
        assert_eq
        push.10901
        assert_eq
        push.9309
        assert_eq
        push.3170
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.5367
        assert_eq
        push.5028
        assert_eq
        push.9207
        assert_eq
        push.8352
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.4259
        assert_eq
        push.8870
        assert_eq
        push.647
        assert_eq
        push.8828
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.9009
        assert_eq
        push.4268
        assert_eq
        push.14
        assert_eq
        push.8566
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.5641
        assert_eq
        push.10390
        assert_eq
        push.8772
        assert_eq
        push.4359
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.1039
        assert_eq
        push.6509
        assert_eq
        push.7103
        assert_eq
        push.1873
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.9303
        assert_eq
        push.2060
        assert_eq
        push.66
        assert_eq
        push.10743
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.8407
        assert_eq
        push.6147
        assert_eq
        push.6608
        assert_eq
        push.10094
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.3098
        assert_eq
        push.8427
        assert_eq
        push.1720
        assert_eq
        push.6074
        assert_eq
        add.1
    
        dup
        push.0.0.0.0
        movup.4
        mem_loadw
        push.1190
        assert_eq
        push.8738
        assert_eq
        push.11930
        assert_eq
        push.56
        assert_eq
        add.1
    
        push.0.0.0.0
        movup.4
        mem_loadw
        push.7831
        assert_eq
        push.11385
        assert_eq
        push.10958
        assert_eq
        push.1343
        assert_eq
    end

    begin
        exec.wrapper
    end
    ";

    let test = build_test!(source, &[]);
    test.get_last_stack_state();
}

#[test]
fn test_poly512_add_zq() {
    let source = "
    use.std::math::poly512

    proc.wrapper.384
        # prepare first polynomial `f`

        push.7618.7764.7271.4394
        loc_storew.127
        dropw
        
        push.240.9007.7416.2384
        loc_storew.126
        dropw
        
        push.151.696.5752.9855
        loc_storew.125
        dropw
        
        push.11254.226.6491.7068
        loc_storew.124
        dropw
        
        push.10516.11999.4160.8221
        loc_storew.123
        dropw
        
        push.5661.2131.1543.1886
        loc_storew.122
        dropw

        
        push.10731.11960.10244.5368
        loc_storew.121
        dropw

        
        push.1223.5240.4765.9963
        loc_storew.120
        dropw

        
        push.10751.2666.9203.7421
        loc_storew.119
        dropw

        
        push.8385.360.12030.6617
        loc_storew.118
        dropw

        
        push.2200.1559.7969.7859
        loc_storew.117
        dropw

        
        push.1688.5958.1035.1013
        loc_storew.116
        dropw

        
        push.6134.5570.5407.6433
        loc_storew.115
        dropw

        
        push.8960.2113.318.5227
        loc_storew.114
        dropw

        
        push.10371.11650.6156.5958
        loc_storew.113
        dropw

        
        push.3686.2823.11955.8012
        loc_storew.112
        dropw

        
        push.6769.9419.3993.4488
        loc_storew.111
        dropw

        
        push.81.190.3011.11793
        loc_storew.110
        dropw

        
        push.9463.3396.2171.3566
        loc_storew.109
        dropw

        
        push.10744.852.10397.2509
        loc_storew.108
        dropw

        
        push.10901.9641.11403.4222
        loc_storew.107
        dropw

        
        push.5217.7112.8609.9784
        loc_storew.106
        dropw

        
        push.8544.1738.3735.11320
        loc_storew.105
        dropw

        
        push.11983.7633.734.2530
        loc_storew.104
        dropw

        
        push.5079.3436.811.9673
        loc_storew.103
        dropw

        
        push.8968.5560.12079.9088
        loc_storew.102
        dropw

        
        push.2836.7454.4608.862
        loc_storew.101
        dropw

        
        push.639.6743.1732.10708
        loc_storew.100
        dropw

        
        push.4875.4161.6301.9212
        loc_storew.99
        dropw
        
        push.11218.67.11674.11861
        loc_storew.98
        dropw
        
        push.7210.5869.5014.6718
        loc_storew.97
        dropw
        
        push.2706.380.2286.3909
        loc_storew.96
        dropw
        
        push.2070.4599.6989.1000
        loc_storew.95
        dropw
        
        push.826.2997.165.6746
        loc_storew.94
        dropw
        
        push.3094.391.2166.7591
        loc_storew.93
        dropw
        
        push.532.1595.11816.195
        loc_storew.92
        dropw
        
        push.10009.8671.9088.6851
        loc_storew.91
        dropw
        
        push.522.11178.5937.3377
        loc_storew.90
        dropw
        
        push.573.12185.9043.8081
        loc_storew.89
        dropw
        
        push.3058.11401.7664.6180
        loc_storew.88
        dropw
        
        push.348.9627.4467.8534
        loc_storew.87
        dropw
        
        push.4289.5381.1181.11304
        loc_storew.86
        dropw
        
        push.6045.2243.11189.7050
        loc_storew.85
        dropw
        
        push.5918.11542.11147.4685
        loc_storew.84
        dropw
        
        push.1817.6002.11775.3084
        loc_storew.83
        dropw
        
        push.3440.3810.7250.7448
        loc_storew.82
        dropw
        
        push.2387.919.2999.4003
        loc_storew.81
        dropw
        
        push.4436.9507.1425.6738
        loc_storew.80
        dropw
        
        push.9287.11417.10830.6660
        loc_storew.79
        dropw
        
        push.7950.1656.1297.476
        loc_storew.78
        dropw
        
        push.283.8597.11086.5239
        loc_storew.77
        dropw
        
        push.589.8488.8795.7944
        loc_storew.76
        dropw
        
        push.3503.9815.11322.8029
        loc_storew.75
        dropw
        
        push.5579.8690.7495.2906
        loc_storew.74
        dropw
        
        push.12064.6795.2785.6564
        loc_storew.73
        dropw
        
        push.3853.3671.1751.4550
        loc_storew.72
        dropw
        
        push.5527.3345.4434.11097
        loc_storew.71
        dropw
        
        push.5401.2248.8989.3040
        loc_storew.70
        dropw
        
        push.11125.2582.6380.5365
        loc_storew.69
        dropw
        
        push.11771.8238.11912.8595
        loc_storew.68
        dropw
        
        push.2163.8230.4250.5354
        loc_storew.67
        dropw
        
        push.6625.3398.1150.8063
        loc_storew.66
        dropw
        
        push.3488.2537.10432.3812
        loc_storew.65
        dropw
        
        push.10661.8269.1789.7507
        loc_storew.64
        dropw
        
        push.5770.6350.5495.8113
        loc_storew.63
        dropw
        
        push.2282.737.650.10641
        loc_storew.62
        dropw
        
        push.9439.10085.3053.5555
        loc_storew.61
        dropw
        
        push.11982.11108.164.840
        loc_storew.60
        dropw
        
        push.5117.5300.4751.11981
        loc_storew.59
        dropw
        
        push.4584.11753.6888.3782
        loc_storew.58
        dropw
        
        push.3948.10542.4745.6791
        loc_storew.57
        dropw
        
        push.434.3061.6757.5192
        loc_storew.56
        dropw
        
        push.3139.8759.4495.10332
        loc_storew.55
        dropw
        
        push.1111.9712.9100.10891
        loc_storew.54
        dropw
        
        push.8147.5643.10068.5768
        loc_storew.53
        dropw
        
        push.2232.8988.9849.2057
        loc_storew.52
        dropw
        
        push.7467.1020.11978.10889
        loc_storew.51
        dropw
        
        push.4118.8060.5659.7640
        loc_storew.50
        dropw
        
        push.6317.10621.10523.755
        loc_storew.49
        dropw
        
        push.5309.1323.10963.886
        loc_storew.48
        dropw
        
        push.11317.3194.7864.8345
        loc_storew.47
        dropw
        
        push.3535.10043.4283.6121
        loc_storew.46
        dropw
        
        push.6200.8954.6105.1139
        loc_storew.45
        dropw
        
        push.2251.11220.6486.8163
        loc_storew.44
        dropw
        
        push.4813.11843.11216.10137
        loc_storew.43
        dropw
        
        push.10813.5679.8204.1648
        loc_storew.42
        dropw
        
        push.11375.8547.9702.7582
        loc_storew.41
        dropw
        
        push.2277.3642.6576.3166
        loc_storew.40
        dropw
        
        push.11404.3420.12135.6431
        loc_storew.39
        dropw
        
        push.9549.10272.3869.7997
        loc_storew.38
        dropw
        
        push.1975.2226.12018.95
        loc_storew.37
        dropw
        
        push.8421.11391.10714.9038
        loc_storew.36
        dropw
        
        push.6620.6649.3572.3789
        loc_storew.35
        dropw
        
        push.5219.4008.11203.9672
        loc_storew.34
        dropw
        
        push.6510.5476.11484.7022
        loc_storew.33
        dropw
        
        push.3490.7500.9936.7836
        loc_storew.32
        dropw
        
        push.11572.10815.10576.3575
        loc_storew.31
        dropw
        
        push.1421.9053.4035.8613
        loc_storew.30
        dropw
        
        push.9806.2186.10907.11868
        loc_storew.29
        dropw
        
        push.7376.9934.5963.2861
        loc_storew.28
        dropw
        
        push.9101.5434.10503.2109
        loc_storew.27
        dropw
        
        push.446.209.1679.4296
        loc_storew.26
        dropw
        
        push.4701.3682.4013.508
        loc_storew.25
        dropw
        
        push.10643.6272.10058.6575
        loc_storew.24
        dropw
        
        push.3016.5032.8623.12217
        loc_storew.23
        dropw
        
        push.91.4684.7233.53
        loc_storew.22
        dropw
        
        push.9335.9009.4008.5022
        loc_storew.21
        dropw
        
        push.9012.9274.2276.415
        loc_storew.20
        dropw
        
        push.5016.10207.940.1750
        loc_storew.19
        dropw
        
        push.7365.10035.7526.9703
        loc_storew.18
        dropw
        
        push.2274.7850.5694.8346
        loc_storew.17
        dropw
        
        push.11501.10018.4533.7010
        loc_storew.16
        dropw
        
        push.10479.9972.11407.12055
        loc_storew.15
        dropw
        
        push.4024.4270.12158.9184
        loc_storew.14
        dropw
        
        push.454.12075.8262.6427
        loc_storew.13
        dropw
        
        push.3104.6247.6381.4144
        loc_storew.12
        dropw
        
        push.10312.2842.2999.6860
        loc_storew.11
        dropw
        
        push.3072.9317.11223.9895
        loc_storew.10
        dropw
        
        push.8506.9148.4979.5304
        loc_storew.9
        dropw
        
        push.8913.623.8621.4430
        loc_storew.8
        dropw
        
        push.8722.94.4069.8477
        loc_storew.7
        dropw
        
        push.4166.9022.10574.5812
        loc_storew.6
        dropw
        
        push.1765.6902.6646.1069
        loc_storew.5
        dropw
        
        push.11125.318.2169.9207
        loc_storew.4
        dropw
        
        push.10176.6543.6207.8256
        loc_storew.3
        dropw
        
        push.5924.1719.8671.3325
        loc_storew.2
        dropw
        
        push.352.11961.8580.1130
        loc_storew.1
        dropw
        
        push.1268.10938.11332.7679
        loc_storew.0
        dropw

        # prepare second polynomial `g`

        push.4548.4624.5028.8016
        loc_storew.255
        dropw


        push.12067.3544.4708.9842
        loc_storew.254
        dropw


        push.11951.11773.6802.2573
        loc_storew.253
        dropw


        push.930.12004.5703.5362
        loc_storew.252
        dropw


        push.1895.321.8224.4004
        loc_storew.251
        dropw


        push.6387.10177.10839.10150
        loc_storew.250
        dropw


        push.1546.67.2120.7084
        loc_storew.249
        dropw


        push.11208.6960.7352.2237
        loc_storew.248
        dropw


        push.1468.9614.3214.4917
        loc_storew.247
        dropw


        push.3730.11893.42.5508
        loc_storew.246
        dropw


        push.10528.10664.4540.4334
        loc_storew.245
        dropw


        push.10039.6362.11533.10943
        loc_storew.244
        dropw


        push.6058.6284.6724.5817
        loc_storew.243
        dropw


        push.3346.10119.11888.7061
        loc_storew.242
        dropw


        push.2161.449.6278.6218
        loc_storew.241
        dropw


        push.8741.9412.568.4188
        loc_storew.240
        dropw


        push.5375.2713.8473.7565
        loc_storew.239
        dropw


        push.12022.11974.9206.675
        loc_storew.238
        dropw


        push.2660.8793.10408.8742
        loc_storew.237
        dropw


        push.1634.11309.1886.9629
        loc_storew.236
        dropw


        push.1170.2691.628.8110
        loc_storew.235
        dropw


        push.7134.5225.3768.2455
        loc_storew.234
        dropw


        push.3638.10582.8430.978
        loc_storew.233
        dropw


        push.159.4468.11693.9863
        loc_storew.232
        dropw


        push.7492.8771.11629.2483
        loc_storew.231
        dropw


        push.3245.6868.410.3289
        loc_storew.230
        dropw


        push.9411.4845.7885.11372
        loc_storew.229
        dropw


        push.11512.5586.10278.1671
        loc_storew.228
        dropw


        push.7374.7956.6238.2929
        loc_storew.227
        dropw


        push.1146.11964.375.596
        loc_storew.226
        dropw


        push.5089.6236.7277.5699
        loc_storew.225
        dropw


        push.9333.11839.10304.8490
        loc_storew.224
        dropw


        push.10003.7707.5631.11261
        loc_storew.223
        dropw


        push.11414.9283.12124.5537
        loc_storew.222
        dropw


        push.9208.11661.10155.4714
        loc_storew.221
        dropw


        push.11829.10645.480.2
        loc_storew.220
        dropw


        push.2341.3538.3118.5742
        loc_storew.219
        dropw


        push.11951.1140.6332.9048
        loc_storew.218
        dropw


        push.11717.390.3289.4537
        loc_storew.217
        dropw


        push.9472.1061.4538.6123
        loc_storew.216
        dropw


        push.12110.2450.7757.3719
        loc_storew.215
        dropw


        push.7777.6827.11293.1227
        loc_storew.214
        dropw


        push.6167.9962.862.5441
        loc_storew.213
        dropw


        push.6432.1136.1049.7677
        loc_storew.212
        dropw


        push.10157.5988.564.9082
        loc_storew.211
        dropw


        push.9087.8756.5034.4828
        loc_storew.210
        dropw


        push.9850.11354.9170.8416
        loc_storew.209
        dropw


        push.7707.2954.11069.5463
        loc_storew.208
        dropw


        push.2888.951.1823.5864
        loc_storew.207
        dropw


        push.4095.10640.11039.11900
        loc_storew.206
        dropw


        push.12027.3424.1194.7090
        loc_storew.205
        dropw


        push.11797.3589.3756.4188
        loc_storew.204
        dropw


        push.8989.2586.745.4185
        loc_storew.203
        dropw


        push.6902.3436.4831.9302
        loc_storew.202
        dropw


        push.342.5361.9688.5866
        loc_storew.201
        dropw


        push.8660.8503.10503.7472
        loc_storew.200
        dropw


        push.6753.8843.7840.1075
        loc_storew.199
        dropw


        push.7148.10194.3305.9269
        loc_storew.198
        dropw


        push.928.9958.5698.6941
        loc_storew.197
        dropw


        push.398.3976.402.3637
        loc_storew.196
        dropw


        push.10333.3918.8008.6998
        loc_storew.195
        dropw


        push.5644.8969.11130.4358
        loc_storew.194
        dropw


        push.8531.9731.1732.8614
        loc_storew.193
        dropw


        push.1638.4129.10651.4636
        loc_storew.192
        dropw


        push.6484.5737.6696.4282
        loc_storew.191
        dropw


        push.10199.11484.11432.1700
        loc_storew.190
        dropw


        push.2997.2059.9191.6774
        loc_storew.189
        dropw


        push.262.1047.11831.11813
        loc_storew.188
        dropw


        push.7396.7081.7598.287
        loc_storew.187
        dropw


        push.7719.715.5332.8615
        loc_storew.186
        dropw


        push.8476.1838.7677.5730
        loc_storew.185
        dropw


        push.11764.9512.5465.7065
        loc_storew.184
        dropw


        push.9256.3625.7864.1878
        loc_storew.183
        dropw


        push.11306.2552.3247.1553
        loc_storew.182
        dropw


        push.4109.6330.2331.6556
        loc_storew.181
        dropw


        push.9844.3366.2444.10139
        loc_storew.180
        dropw


        push.4904.11135.21.1434
        loc_storew.179
        dropw


        push.8147.4344.7010.4334
        loc_storew.178
        dropw


        push.5825.1749.1812.11928
        loc_storew.177
        dropw


        push.6994.10765.1291.11536
        loc_storew.176
        dropw


        push.817.9423.4401.4131
        loc_storew.175
        dropw


        push.8418.2295.7810.6016
        loc_storew.174
        dropw


        push.5960.3090.6202.10904
        loc_storew.173
        dropw


        push.10116.1217.5825.4040
        loc_storew.172
        dropw


        push.7542.295.1483.2531
        loc_storew.171
        dropw


        push.1419.6772.4177.10694
        loc_storew.170
        dropw


        push.1070.3694.2461.4824
        loc_storew.169
        dropw


        push.10013.8551.5733.9109
        loc_storew.168
        dropw


        push.555.8809.361.5956
        loc_storew.167
        dropw


        push.2828.1914.8436.4285
        loc_storew.166
        dropw


        push.10170.10037.466.12019
        loc_storew.165
        dropw


        push.3614.960.1647.3507
        loc_storew.164
        dropw


        push.5982.5581.8632.8567
        loc_storew.163
        dropw


        push.7333.8151.1269.2532
        loc_storew.162
        dropw


        push.5950.6912.645.5552
        loc_storew.161
        dropw


        push.8935.4564.2260.4497
        loc_storew.160
        dropw


        push.516.1390.1984.8812
        loc_storew.159
        dropw


        push.10924.3322.8284.3744
        loc_storew.158
        dropw


        push.2531.10370.1642.261
        loc_storew.157
        dropw


        push.4957.2504.6323.9432
        loc_storew.156
        dropw


        push.3055.7149.1848.10436
        loc_storew.155
        dropw


        push.12190.109.10759.7918
        loc_storew.154
        dropw


        push.7749.8442.8212.11481
        loc_storew.153
        dropw


        push.1367.5884.2374.5696
        loc_storew.152
        dropw


        push.9618.7168.3517.12273
        loc_storew.151
        dropw


        push.179.7405.5187.247
        loc_storew.150
        dropw


        push.2783.3469.8204.7272
        loc_storew.149
        dropw


        push.3246.2753.9921.11814
        loc_storew.148
        dropw


        push.7508.2028.11394.10636
        loc_storew.147
        dropw


        push.5062.2304.4768.2506
        loc_storew.146
        dropw


        push.10144.4449.6804.3899
        loc_storew.145
        dropw


        push.810.2389.8029.5419
        loc_storew.144
        dropw


        push.1716.2405.757.12251
        loc_storew.143
        dropw


        push.8055.8028.12152.2837
        loc_storew.142
        dropw


        push.11778.131.4064.6032
        loc_storew.141
        dropw


        push.9020.6084.5749.8269
        loc_storew.140
        dropw


        push.2082.9402.9212.5295
        loc_storew.139
        dropw


        push.9119.2980.1388.2516
        loc_storew.138
        dropw


        push.3937.3082.7261.6922
        loc_storew.137
        dropw


        push.3461.11642.3419.8030
        loc_storew.136
        dropw


        push.3723.12275.8021.3280
        loc_storew.135
        dropw


        push.7930.3517.1899.6648
        loc_storew.134
        dropw


        push.10416.5186.5780.11250
        loc_storew.133
        dropw


        push.1546.12223.10229.2986
        loc_storew.132
        dropw


        push.2195.5681.6142.3882
        loc_storew.131
        dropw


        push.6215.10569.3862.9191
        loc_storew.130
        dropw


        push.12233.359.3551.11099
        loc_storew.129
        dropw


        push.10946.1331.904.4458
        loc_storew.128
        dropw


        # prepare argument ( absolute memory addresses ) for adding two polynomials

        locaddr.383 # output
        locaddr.255 # input 1
        locaddr.127 # input 0

        # perform polynomial addition, when two polynomials are provided
        # as absolute memory addresses on the stack

        exec.poly512::add_zq

        # check for functional correctness ( using known answer test )

        locaddr.383

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.121
        assert_eq
        push.10
        assert_eq
        push.99
        assert_eq
        push.12166
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12226
        assert_eq
        push.12124
        assert_eq
        push.262
        assert_eq
        push.18
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.139
        assert_eq
        push.265
        assert_eq
        push.180
        assert_eq
        push.12102
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.141
        assert_eq
        push.12194
        assert_eq
        push.12230
        assert_eq
        push.12184
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12225
        assert_eq
        push.95
        assert_eq
        push.31
        assert_eq
        push.122
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12036
        assert_eq
        push.93
        assert_eq
        push.19
        assert_eq
        push.12048
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.163
        assert_eq
        push.75
        assert_eq
        push.12027
        assert_eq
        push.12277
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12200
        assert_eq
        push.12117
        assert_eq
        push.12200
        assert_eq
        push.142
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.49
        assert_eq
        push.128
        assert_eq
        push.12280
        assert_eq
        push.12219
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12125
        assert_eq
        push.12072
        assert_eq
        push.12253
        assert_eq
        push.12115
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12193
        assert_eq
        push.220
        assert_eq
        push.12223
        assert_eq
        push.439
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11956
        assert_eq
        push.279
        assert_eq
        push.31
        assert_eq
        push.11727
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12250
        assert_eq
        push.12131
        assert_eq
        push.11854
        assert_eq
        push.12192
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12288
        assert_eq
        push.12206
        assert_eq
        push.12232
        assert_eq
        push.17
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12176
        assert_eq
        push.145
        assert_eq
        push.12099
        assert_eq
        push.243
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12200
        assert_eq
        push.234
        assert_eq
        push.12235
        assert_eq
        push.138
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12053
        assert_eq
        push.177
        assert_eq
        push.12132
        assert_eq
        push.12144
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.179
        assert_eq
        push.12217
        assert_eq
        push.12164
        assert_eq
        push.12103
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.19
        assert_eq
        push.290
        assert_eq
        push.12189
        assert_eq
        push.12123
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12138
        assert_eq
        push.12283
        assert_eq
        push.12161
        assert_eq
        push.89
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.43
        assert_eq
        push.12031
        assert_eq
        push.43
        assert_eq
        push.12071
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12239
        assert_eq
        push.88
        assert_eq
        push.48
        assert_eq
        push.62
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.9
        assert_eq
        push.12165
        assert_eq
        push.31
        assert_eq
        push.12182
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.104
        assert_eq
        push.138
        assert_eq
        push.12101
        assert_eq
        push.12142
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12156
        assert_eq
        push.151
        assert_eq
        push.12207
        assert_eq
        push.282
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.88
        assert_eq
        push.200
        assert_eq
        push.139
        assert_eq
        push.12213
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12234
        assert_eq
        push.204
        assert_eq
        push.10
        assert_eq
        push.12247
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.90
        assert_eq
        push.12010
        assert_eq
        push.40
        assert_eq
        push.12151
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12141
        assert_eq
        push.250
        assert_eq
        push.12117
        assert_eq
        push.12249
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.168
        assert_eq
        push.12049
        assert_eq
        push.12031
        assert_eq
        push.75
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.128
        assert_eq
        push.2
        assert_eq
        push.12105
        assert_eq
        push.10
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.110
        assert_eq
        push.301
        assert_eq
        push.12219
        assert_eq
        push.12039
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12261
        assert_eq
        push.331
        assert_eq
        push.17
        assert_eq
        push.12073
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12283
        assert_eq
        push.0
        assert_eq
        push.12280
        assert_eq
        push.12240
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.16
        assert_eq
        push.32
        assert_eq
        push.12052
        assert_eq
        push.13
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.197
        assert_eq
        push.7
        assert_eq
        push.12240
        assert_eq
        push.72
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.304
        assert_eq
        push.12206
        assert_eq
        push.12209
        assert_eq
        push.61
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.136
        assert_eq
        push.12269
        assert_eq
        push.29
        assert_eq
        push.184
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.329
        assert_eq
        push.43
        assert_eq
        push.286
        assert_eq
        push.1
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.14
        assert_eq
        push.12202
        assert_eq
        push.173
        assert_eq
        push.241
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12253
        assert_eq
        push.12224
        assert_eq
        push.12077
        assert_eq
        push.169
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.242
        assert_eq
        push.185
        assert_eq
        push.12208
        assert_eq
        push.12066
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.202
        assert_eq
        push.12051
        assert_eq
        push.12205
        assert_eq
        push.12212
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.73
        assert_eq
        push.12196
        assert_eq
        push.389
        assert_eq
        push.61
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12166
        assert_eq
        push.50
        assert_eq
        push.11990
        assert_eq
        push.11974
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12276
        assert_eq
        push.12284
        assert_eq
        push.277
        assert_eq
        push.238
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.130
        assert_eq
        push.12169
        assert_eq
        push.12273
        assert_eq
        push.12237
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12201
        assert_eq
        push.205
        assert_eq
        push.172
        assert_eq
        push.12143
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.235
        assert_eq
        push.364
        assert_eq
        push.79
        assert_eq
        push.12175
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.87
        assert_eq
        push.47
        assert_eq
        push.7
        assert_eq
        push.12045
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.40
        assert_eq
        push.12280
        assert_eq
        push.12021
        assert_eq
        push.21
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12132
        assert_eq
        push.262
        assert_eq
        push.12077
        assert_eq
        push.97
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12214
        assert_eq
        push.12067
        assert_eq
        push.112
        assert_eq
        push.203
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12208
        assert_eq
        push.37
        assert_eq
        push.12126
        assert_eq
        push.192
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.141
        assert_eq
        push.184
        assert_eq
        push.12156
        assert_eq
        push.117
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12022
        assert_eq
        push.12254
        assert_eq
        push.12174
        assert_eq
        push.224
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12172
        assert_eq
        push.12274
        assert_eq
        push.12188
        assert_eq
        push.12280
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.20
        assert_eq
        push.5
        assert_eq
        push.153
        assert_eq
        push.260
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.17
        assert_eq
        push.12078
        assert_eq
        push.251
        assert_eq
        push.12053
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12232
        assert_eq
        push.25
        assert_eq
        push.12214
        assert_eq
        push.12169
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.63
        assert_eq
        push.12258
        assert_eq
        push.12148
        assert_eq
        push.207
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.132
        assert_eq
        push.12280
        assert_eq
        push.78
        assert_eq
        push.12269
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.137
        assert_eq
        push.12164
        assert_eq
        push.12268
        assert_eq
        push.12019
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12143
        assert_eq
        push.151
        assert_eq
        push.109
        assert_eq
        push.10
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.106
        assert_eq
        push.12191
        assert_eq
        push.12087
        assert_eq
        push.12254
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.52
        assert_eq
        push.12082
        assert_eq
        push.12221
        assert_eq
        push.192
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.40
        assert_eq
        push.12244
        assert_eq
        push.12144
        assert_eq
        push.147
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.364
        assert_eq
        push.11995
        assert_eq
        push.12155
        assert_eq
        push.12244
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12268
        assert_eq
        push.60
        assert_eq
        push.92
        assert_eq
        push.224
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.108
        assert_eq
        push.12220
        assert_eq
        push.179
        assert_eq
        push.14
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.232
        assert_eq
        push.133
        assert_eq
        push.91
        assert_eq
        push.135
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12257
        assert_eq
        push.12222
        assert_eq
        push.284
        assert_eq
        push.12198
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12210
        assert_eq
        push.70
        assert_eq
        push.95
        assert_eq
        push.106
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.155
        assert_eq
        push.58
        assert_eq
        push.12264
        assert_eq
        push.128
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.35
        assert_eq
        push.110
        assert_eq
        push.11973
        assert_eq
        push.12256
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12196
        assert_eq
        push.4
        assert_eq
        push.65
        assert_eq
        push.12076
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.34
        assert_eq
        push.11999
        assert_eq
        push.12155
        assert_eq
        push.82
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11974
        assert_eq
        push.380
        assert_eq
        push.115
        assert_eq
        push.12265
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.394
        assert_eq
        push.46
        assert_eq
        push.81
        assert_eq
        push.12142
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.133
        assert_eq
        push.12254
        assert_eq
        push.12088
        assert_eq
        push.14
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.187
        assert_eq
        push.12265
        assert_eq
        push.328
        assert_eq
        push.12134
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12137
        assert_eq
        push.12093
        assert_eq
        push.49
        assert_eq
        push.11953
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12043
        assert_eq
        push.18
        assert_eq
        push.12044
        assert_eq
        push.12160
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12203
        assert_eq
        push.22
        assert_eq
        push.148
        assert_eq
        push.78
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.379
        assert_eq
        push.410
        assert_eq
        push.12138
        assert_eq
        push.66
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.53
        assert_eq
        push.92
        assert_eq
        push.162
        assert_eq
        push.12232
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.117
        assert_eq
        push.12163
        assert_eq
        push.12241
        assert_eq
        push.156
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12275
        assert_eq
        push.20
        assert_eq
        push.12193
        assert_eq
        push.1
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.98
        assert_eq
        push.207
        assert_eq
        push.12229
        assert_eq
        push.11959
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12282
        assert_eq
        push.16
        assert_eq
        push.12186
        assert_eq
        push.88
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12114
        assert_eq
        push.195
        assert_eq
        push.12263
        assert_eq
        push.12145
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.256
        assert_eq
        push.72
        assert_eq
        push.62
        assert_eq
        push.12035
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.67
        assert_eq
        push.12204
        assert_eq
        push.12230
        assert_eq
        push.313
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12204
        assert_eq
        push.183
        assert_eq
        push.12159
        assert_eq
        push.263
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.285
        assert_eq
        push.12129
        assert_eq
        push.99
        assert_eq
        push.171
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.44
        assert_eq
        push.12196
        assert_eq
        push.12064
        assert_eq
        push.136
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.98
        assert_eq
        push.271
        assert_eq
        push.12205
        assert_eq
        push.12088
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.68
        assert_eq
        push.30
        assert_eq
        push.86
        assert_eq
        push.56
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12129
        assert_eq
        push.260
        assert_eq
        push.267
        assert_eq
        push.48
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4
        assert_eq
        push.12286
        assert_eq
        push.149
        assert_eq
        push.44
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.256
        assert_eq
        push.62
        assert_eq
        push.294
        assert_eq
        push.12156
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12214
        assert_eq
        push.149
        assert_eq
        push.318
        assert_eq
        push.347
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11989
        assert_eq
        push.12225
        assert_eq
        push.12124
        assert_eq
        push.161
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12271
        assert_eq
        push.143
        assert_eq
        push.12156
        assert_eq
        push.12010
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12201
        assert_eq
        push.12140
        assert_eq
        push.12200
        assert_eq
        push.345
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.300
        assert_eq
        push.131
        assert_eq
        push.12089
        assert_eq
        push.270
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5
        assert_eq
        push.12212
        assert_eq
        push.189
        assert_eq
        push.12118
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12229
        assert_eq
        push.12197
        assert_eq
        push.12027
        assert_eq
        push.12258
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.97
        assert_eq
        push.45
        assert_eq
        push.12235
        assert_eq
        push.235
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12209
        assert_eq
        push.5
        assert_eq
        push.50
        assert_eq
        push.138
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12245
        assert_eq
        push.209
        assert_eq
        push.10
        assert_eq
        push.129
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.140
        assert_eq
        push.273
        assert_eq
        push.118
        assert_eq
        push.22
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12017
        assert_eq
        push.12164
        assert_eq
        push.88
        assert_eq
        push.12195
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12021
        assert_eq
        push.12021
        assert_eq
        push.9
        assert_eq
        push.12079
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.170
        assert_eq
        push.37
        assert_eq
        push.12206
        assert_eq
        push.12232
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.124
        assert_eq
        push.12130
        assert_eq
        push.42
        assert_eq
        push.12124
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12155
        assert_eq
        push.12211
        assert_eq
        push.12244
        assert_eq
        push.105
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.122
        assert_eq
        push.322
        assert_eq
        push.8
        assert_eq
        push.12191
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12226
        assert_eq
        push.12240
        assert_eq
        push.12230
        assert_eq
        push.154
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.171
        assert_eq
        push.12040
        assert_eq
        push.12265
        assert_eq
        push.85
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11757
        assert_eq
        push.12090
        assert_eq
        push.80
        assert_eq
        push.156
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.171
        assert_eq
        push.184
        assert_eq
        push.250
        assert_eq
        push.12096
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.30
        assert_eq
        push.137
        assert_eq
        push.12088
        assert_eq
        push.12181
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12193
        assert_eq
        push.109
        assert_eq
        push.252
        assert_eq
        push.382
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12138
        assert_eq
        push.60
        assert_eq
        push.12224
        assert_eq
        push.82
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.227
        assert_eq
        push.244
        assert_eq
        push.12288
        assert_eq
        push.12139
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12229
        assert_eq
        push.12131
        assert_eq
        push.31
        assert_eq
        push.296
        assert_eq
		add.1
    
        push.0.0.0.0
        movup.4
        mem_loadw
        push.12137
        assert_eq
        push.12236
        assert_eq
        push.12269
        assert_eq
        push.12214
        assert_eq
    end

    begin
        exec.wrapper
    end
    ";

    let test = build_test!(source, &[]);
    test.get_last_stack_state();
}

#[test]
fn test_poly512_neg_zq() {
    let source = "
    use.std::math::poly512

    proc.wrapper.256
        # prepare polynomial `f`

        push.7741.7665.7261.4273
        loc_storew.127
        dropw

        push.222.8745.7581.2447
        loc_storew.126
        dropw

        push.338.516.5487.9716
        loc_storew.125
        dropw

        push.11359.285.6586.6927
        loc_storew.124
        dropw

        push.10394.11968.4065.8285
        loc_storew.123
        dropw

        push.5902.2112.1450.2139
        loc_storew.122
        dropw


        push.10743.12222.10169.5205
        loc_storew.121
        dropw


        push.1081.5329.4937.10052
        loc_storew.120
        dropw


        push.10821.2675.9075.7372
        loc_storew.119
        dropw


        push.8559.396.12247.6781
        loc_storew.118
        dropw


        push.1761.1625.7749.7955
        loc_storew.117
        dropw


        push.2250.5927.756.1346
        loc_storew.116
        dropw


        push.6231.6005.5565.6472
        loc_storew.115
        dropw


        push.8943.2170.401.5228
        loc_storew.114
        dropw


        push.10128.11840.6011.6071
        loc_storew.113
        dropw


        push.3548.2877.11721.8101
        loc_storew.112
        dropw


        push.6914.9576.3816.4724
        loc_storew.111
        dropw


        push.267.315.3083.11614
        loc_storew.110
        dropw


        push.9629.3496.1881.3547
        loc_storew.109
        dropw


        push.10655.980.10403.2660
        loc_storew.108
        dropw


        push.11119.9598.11661.4179
        loc_storew.107
        dropw


        push.5155.7064.8521.9834
        loc_storew.106
        dropw


        push.8651.1707.3859.11311
        loc_storew.105
        dropw


        push.12130.7821.596.2426
        loc_storew.104
        dropw


        push.4797.3518.660.9806
        loc_storew.103
        dropw


        push.9044.5421.11879.9000
        loc_storew.102
        dropw


        push.2878.7444.4404.917
        loc_storew.101
        dropw


        push.777.6703.2011.10618
        loc_storew.100
        dropw


        push.4915.4333.6051.9360
        loc_storew.99
        dropw

        push.11143.325.11914.11693
        loc_storew.98
        dropw

        push.7200.6053.5012.6590
        loc_storew.97
        dropw

        push.2956.450.1985.3799
        loc_storew.96
        dropw

        push.2286.4582.6658.1028
        loc_storew.95
        dropw

        push.875.3006.165.6752
        loc_storew.94
        dropw

        push.3081.628.2134.7575
        loc_storew.93
        dropw

        push.460.1644.11809.12287
        loc_storew.92
        dropw

        push.9948.8751.9171.6547
        loc_storew.91
        dropw

        push.338.11149.5957.3241
        loc_storew.90
        dropw

        push.572.11899.9000.7752
        loc_storew.89
        dropw

        push.2817.11228.7751.6166
        loc_storew.88
        dropw

        push.179.9839.4532.8570
        loc_storew.87
        dropw

        push.4512.5462.996.11062
        loc_storew.86
        dropw

        push.6122.2327.11427.6848
        loc_storew.85
        dropw

        push.5857.11153.11240.4612
        loc_storew.84
        dropw

        push.2132.6301.11725.3207
        loc_storew.83
        dropw

        push.3202.3533.7255.7461
        loc_storew.82
        dropw

        push.2439.935.3119.3873
        loc_storew.81
        dropw

        push.4582.9335.1220.6826
        loc_storew.80
        dropw

        push.9401.11338.10466.6425
        loc_storew.79
        dropw

        push.8194.1649.1250.389
        loc_storew.78
        dropw

        push.262.8865.11095.5199
        loc_storew.77
        dropw

        push.492.8700.8533.8101
        loc_storew.76
        dropw

        push.3300.9703.11544.8104
        loc_storew.75
        dropw

        push.5387.8853.7458.2987
        loc_storew.74
        dropw

        push.11947.6928.2601.6423
        loc_storew.73
        dropw

        push.3629.3786.1786.4817
        loc_storew.72
        dropw

        push.5536.3446.4449.11214
        loc_storew.71
        dropw

        push.5141.2095.8984.3020
        loc_storew.70
        dropw

        push.11361.2331.6591.5348
        loc_storew.69
        dropw

        push.11891.8313.11887.8652
        loc_storew.68
        dropw

        push.1956.8371.4281.5291
        loc_storew.67
        dropw

        push.6645.3320.1159.7931
        loc_storew.66
        dropw

        push.3758.2558.10557.3675
        loc_storew.65
        dropw

        push.10651.8160.1638.7653
        loc_storew.64
        dropw

        push.5805.6552.5593.8007
        loc_storew.63
        dropw

        push.2090.805.857.10589
        loc_storew.62
        dropw

        push.9292.10230.3098.5515
        loc_storew.61
        dropw

        push.12027.11242.458.476
        loc_storew.60
        dropw

        push.4893.5208.4691.12002
        loc_storew.59
        dropw

        push.4570.11574.6957.3674
        loc_storew.58
        dropw

        push.3813.10451.4612.6559
        loc_storew.57
        dropw

        push.525.2777.6824.5224
        loc_storew.56
        dropw

        push.3033.8664.4425.10411
        loc_storew.55
        dropw

        push.983.9737.9042.10736
        loc_storew.54
        dropw

        push.8180.5959.9958.5733
        loc_storew.53
        dropw

        push.2445.8923.9845.2150
        loc_storew.52
        dropw

        push.7385.1154.12268.10855
        loc_storew.51
        dropw

        push.4142.7945.5279.7955
        loc_storew.50
        dropw

        push.6464.10540.10477.361
        loc_storew.49
        dropw

        push.5295.1524.10998.753
        loc_storew.48
        dropw

        push.11472.2866.7888.8158
        loc_storew.47
        dropw

        push.3871.9994.4479.6273
        loc_storew.46
        dropw

        push.6329.9199.6087.1385
        loc_storew.45
        dropw

        push.2173.11072.6464.8249
        loc_storew.44
        dropw

        push.4747.11994.10806.9758
        loc_storew.43
        dropw

        push.10870.5517.8112.1595
        loc_storew.42
        dropw

        push.11219.8595.9828.7465
        loc_storew.41
        dropw

        push.2276.3738.6556.3180
        loc_storew.40
        dropw

        push.11734.3480.11928.6333
        loc_storew.39
        dropw

        push.9461.10375.3853.8004
        loc_storew.38
        dropw

        push.2119.2252.11823.270
        loc_storew.37
        dropw

        push.8675.11329.10642.8782
        loc_storew.36
        dropw

        push.6307.6708.3657.3722
        loc_storew.35
        dropw

        push.4956.4138.11020.9757
        loc_storew.34
        dropw

        push.6339.5377.11644.6737
        loc_storew.33
        dropw

        push.3354.7725.10029.7792
        loc_storew.32
        dropw

        push.11773.10899.10305.3477
        loc_storew.31
        dropw

        push.1365.8967.4005.8545
        loc_storew.30
        dropw

        push.9758.1919.10647.12028
        loc_storew.29
        dropw

        push.7332.9785.5966.2857
        loc_storew.28
        dropw

        push.9234.5140.10441.1853
        loc_storew.27
        dropw

        push.99.12180.1530.4371
        loc_storew.26
        dropw

        push.4540.3847.4077.808
        loc_storew.25
        dropw

        push.10922.6405.9915.6593
        loc_storew.24
        dropw

        push.2671.5121.8772.16
        loc_storew.23
        dropw

        push.12110.4884.7102.12042
        loc_storew.22
        dropw

        push.9506.8820.4085.5017
        loc_storew.21
        dropw

        push.9043.9536.2368.475
        loc_storew.20
        dropw

        push.4781.10261.895.1653
        loc_storew.19
        dropw

        push.7227.9985.7521.9783
        loc_storew.18
        dropw

        push.2145.7840.5485.8390
        loc_storew.17
        dropw

        push.11479.9900.4260.6870
        loc_storew.16
        dropw

        push.10573.9884.11532.38
        loc_storew.15
        dropw

        push.4234.4261.137.9452
        loc_storew.14
        dropw

        push.511.12158.8225.6257
        loc_storew.13
        dropw

        push.3269.6205.6540.4020
        loc_storew.12
        dropw

        push.10207.2887.3077.6994
        loc_storew.11
        dropw

        push.3170.9309.10901.9773
        loc_storew.10
        dropw

        push.8352.9207.5028.5367
        loc_storew.9
        dropw

        push.8828.647.8870.4259
        loc_storew.8
        dropw

        push.8566.14.4268.9009
        loc_storew.7
        dropw

        push.4359.8772.10390.5641
        loc_storew.6
        dropw

        push.1873.7103.6509.1039
        loc_storew.5
        dropw

        push.10743.66.2060.9303
        loc_storew.4
        dropw

        push.10094.6608.6147.8407
        loc_storew.3
        dropw

        push.6074.1720.8427.3098
        loc_storew.2
        dropw

        push.56.11930.8738.1190
        loc_storew.1
        dropw

        push.1343.10958.11385.7831
        loc_storew.0
        dropw

        # prepare argument ( absolute memory address ) for negating one polynomial

        locaddr.255 # output start address
        locaddr.127 # input start address

        # perform polynomial negation, when one polynomial is provided
        # as absolute memory address on the stack, along with resulting
        # polynomial's starting memory address

        exec.poly512::neg_zq

        # check for functional correctness ( using known answer test )

        locaddr.255

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.8016
        assert_eq
        push.5028
        assert_eq
        push.4624
        assert_eq
        push.4548
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.9842
        assert_eq
        push.4708
        assert_eq
        push.3544
        assert_eq
        push.12067
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.2573
        assert_eq
        push.6802
        assert_eq
        push.11773
        assert_eq
        push.11951
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5362
        assert_eq
        push.5703
        assert_eq
        push.12004
        assert_eq
        push.930
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4004
        assert_eq
        push.8224
        assert_eq
        push.321
        assert_eq
        push.1895
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.10150
        assert_eq
        push.10839
        assert_eq
        push.10177
        assert_eq
        push.6387
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.7084
        assert_eq
        push.2120
        assert_eq
        push.67
        assert_eq
        push.1546
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.2237
        assert_eq
        push.7352
        assert_eq
        push.6960
        assert_eq
        push.11208
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4917
        assert_eq
        push.3214
        assert_eq
        push.9614
        assert_eq
        push.1468
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5508
        assert_eq
        push.42
        assert_eq
        push.11893
        assert_eq
        push.3730
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4334
        assert_eq
        push.4540
        assert_eq
        push.10664
        assert_eq
        push.10528
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.10943
        assert_eq
        push.11533
        assert_eq
        push.6362
        assert_eq
        push.10039
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5817
        assert_eq
        push.6724
        assert_eq
        push.6284
        assert_eq
        push.6058
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.7061
        assert_eq
        push.11888
        assert_eq
        push.10119
        assert_eq
        push.3346
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.6218
        assert_eq
        push.6278
        assert_eq
        push.449
        assert_eq
        push.2161
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4188
        assert_eq
        push.568
        assert_eq
        push.9412
        assert_eq
        push.8741
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.7565
        assert_eq
        push.8473
        assert_eq
        push.2713
        assert_eq
        push.5375
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.675
        assert_eq
        push.9206
        assert_eq
        push.11974
        assert_eq
        push.12022
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.8742
        assert_eq
        push.10408
        assert_eq
        push.8793
        assert_eq
        push.2660
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.9629
        assert_eq
        push.1886
        assert_eq
        push.11309
        assert_eq
        push.1634
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.8110
        assert_eq
        push.628
        assert_eq
        push.2691
        assert_eq
        push.1170
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.2455
        assert_eq
        push.3768
        assert_eq
        push.5225
        assert_eq
        push.7134
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.978
        assert_eq
        push.8430
        assert_eq
        push.10582
        assert_eq
        push.3638
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.9863
        assert_eq
        push.11693
        assert_eq
        push.4468
        assert_eq
        push.159
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.2483
        assert_eq
        push.11629
        assert_eq
        push.8771
        assert_eq
        push.7492
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.3289
        assert_eq
        push.410
        assert_eq
        push.6868
        assert_eq
        push.3245
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11372
        assert_eq
        push.7885
        assert_eq
        push.4845
        assert_eq
        push.9411
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.1671
        assert_eq
        push.10278
        assert_eq
        push.5586
        assert_eq
        push.11512
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.2929
        assert_eq
        push.6238
        assert_eq
        push.7956
        assert_eq
        push.7374
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.596
        assert_eq
        push.375
        assert_eq
        push.11964
        assert_eq
        push.1146
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5699
        assert_eq
        push.7277
        assert_eq
        push.6236
        assert_eq
        push.5089
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.8490
        assert_eq
        push.10304
        assert_eq
        push.11839
        assert_eq
        push.9333
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11261
        assert_eq
        push.5631
        assert_eq
        push.7707
        assert_eq
        push.10003
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5537
        assert_eq
        push.12124
        assert_eq
        push.9283
        assert_eq
        push.11414
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4714
        assert_eq
        push.10155
        assert_eq
        push.11661
        assert_eq
        push.9208
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.2
        assert_eq
        push.480
        assert_eq
        push.10645
        assert_eq
        push.11829
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5742
        assert_eq
        push.3118
        assert_eq
        push.3538
        assert_eq
        push.2341
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.9048
        assert_eq
        push.6332
        assert_eq
        push.1140
        assert_eq
        push.11951
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4537
        assert_eq
        push.3289
        assert_eq
        push.390
        assert_eq
        push.11717
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.6123
        assert_eq
        push.4538
        assert_eq
        push.1061
        assert_eq
        push.9472
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.3719
        assert_eq
        push.7757
        assert_eq
        push.2450
        assert_eq
        push.12110
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.1227
        assert_eq
        push.11293
        assert_eq
        push.6827
        assert_eq
        push.7777
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5441
        assert_eq
        push.862
        assert_eq
        push.9962
        assert_eq
        push.6167
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.7677
        assert_eq
        push.1049
        assert_eq
        push.1136
        assert_eq
        push.6432
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.9082
        assert_eq
        push.564
        assert_eq
        push.5988
        assert_eq
        push.10157
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4828
        assert_eq
        push.5034
        assert_eq
        push.8756
        assert_eq
        push.9087
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.8416
        assert_eq
        push.9170
        assert_eq
        push.11354
        assert_eq
        push.9850
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5463
        assert_eq
        push.11069
        assert_eq
        push.2954
        assert_eq
        push.7707
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5864
        assert_eq
        push.1823
        assert_eq
        push.951
        assert_eq
        push.2888
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11900
        assert_eq
        push.11039
        assert_eq
        push.10640
        assert_eq
        push.4095
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.7090
        assert_eq
        push.1194
        assert_eq
        push.3424
        assert_eq
        push.12027
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4188
        assert_eq
        push.3756
        assert_eq
        push.3589
        assert_eq
        push.11797
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4185
        assert_eq
        push.745
        assert_eq
        push.2586
        assert_eq
        push.8989
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.9302
        assert_eq
        push.4831
        assert_eq
        push.3436
        assert_eq
        push.6902
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5866
        assert_eq
        push.9688
        assert_eq
        push.5361
        assert_eq
        push.342
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.7472
        assert_eq
        push.10503
        assert_eq
        push.8503
        assert_eq
        push.8660
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.1075
        assert_eq
        push.7840
        assert_eq
        push.8843
        assert_eq
        push.6753
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.9269
        assert_eq
        push.3305
        assert_eq
        push.10194
        assert_eq
        push.7148
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.6941
        assert_eq
        push.5698
        assert_eq
        push.9958
        assert_eq
        push.928
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.3637
        assert_eq
        push.402
        assert_eq
        push.3976
        assert_eq
        push.398
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.6998
        assert_eq
        push.8008
        assert_eq
        push.3918
        assert_eq
        push.10333
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4358
        assert_eq
        push.11130
        assert_eq
        push.8969
        assert_eq
        push.5644
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.8614
        assert_eq
        push.1732
        assert_eq
        push.9731
        assert_eq
        push.8531
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4636
        assert_eq
        push.10651
        assert_eq
        push.4129
        assert_eq
        push.1638
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4282
        assert_eq
        push.6696
        assert_eq
        push.5737
        assert_eq
        push.6484
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.1700
        assert_eq
        push.11432
        assert_eq
        push.11484
        assert_eq
        push.10199
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.6774
        assert_eq
        push.9191
        assert_eq
        push.2059
        assert_eq
        push.2997
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11813
        assert_eq
        push.11831
        assert_eq
        push.1047
        assert_eq
        push.262
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.287
        assert_eq
        push.7598
        assert_eq
        push.7081
        assert_eq
        push.7396
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.8615
        assert_eq
        push.5332
        assert_eq
        push.715
        assert_eq
        push.7719
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5730
        assert_eq
        push.7677
        assert_eq
        push.1838
        assert_eq
        push.8476
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.7065
        assert_eq
        push.5465
        assert_eq
        push.9512
        assert_eq
        push.11764
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.1878
        assert_eq
        push.7864
        assert_eq
        push.3625
        assert_eq
        push.9256
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.1553
        assert_eq
        push.3247
        assert_eq
        push.2552
        assert_eq
        push.11306
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.6556
        assert_eq
        push.2331
        assert_eq
        push.6330
        assert_eq
        push.4109
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.10139
        assert_eq
        push.2444
        assert_eq
        push.3366
        assert_eq
        push.9844
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.1434
        assert_eq
        push.21
        assert_eq
        push.11135
        assert_eq
        push.4904
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4334
        assert_eq
        push.7010
        assert_eq
        push.4344
        assert_eq
        push.8147
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11928
        assert_eq
        push.1812
        assert_eq
        push.1749
        assert_eq
        push.5825
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11536
        assert_eq
        push.1291
        assert_eq
        push.10765
        assert_eq
        push.6994
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4131
        assert_eq
        push.4401
        assert_eq
        push.9423
        assert_eq
        push.817
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.6016
        assert_eq
        push.7810
        assert_eq
        push.2295
        assert_eq
        push.8418
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.10904
        assert_eq
        push.6202
        assert_eq
        push.3090
        assert_eq
        push.5960
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4040
        assert_eq
        push.5825
        assert_eq
        push.1217
        assert_eq
        push.10116
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.2531
        assert_eq
        push.1483
        assert_eq
        push.295
        assert_eq
        push.7542
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.10694
        assert_eq
        push.4177
        assert_eq
        push.6772
        assert_eq
        push.1419
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4824
        assert_eq
        push.2461
        assert_eq
        push.3694
        assert_eq
        push.1070
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.9109
        assert_eq
        push.5733
        assert_eq
        push.8551
        assert_eq
        push.10013
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5956
        assert_eq
        push.361
        assert_eq
        push.8809
        assert_eq
        push.555
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4285
        assert_eq
        push.8436
        assert_eq
        push.1914
        assert_eq
        push.2828
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12019
        assert_eq
        push.466
        assert_eq
        push.10037
        assert_eq
        push.10170
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.3507
        assert_eq
        push.1647
        assert_eq
        push.960
        assert_eq
        push.3614
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.8567
        assert_eq
        push.8632
        assert_eq
        push.5581
        assert_eq
        push.5982
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.2532
        assert_eq
        push.1269
        assert_eq
        push.8151
        assert_eq
        push.7333
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5552
        assert_eq
        push.645
        assert_eq
        push.6912
        assert_eq
        push.5950
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4497
        assert_eq
        push.2260
        assert_eq
        push.4564
        assert_eq
        push.8935
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.8812
        assert_eq
        push.1984
        assert_eq
        push.1390
        assert_eq
        push.516
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.3744
        assert_eq
        push.8284
        assert_eq
        push.3322
        assert_eq
        push.10924
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.261
        assert_eq
        push.1642
        assert_eq
        push.10370
        assert_eq
        push.2531
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.9432
        assert_eq
        push.6323
        assert_eq
        push.2504
        assert_eq
        push.4957
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.10436
        assert_eq
        push.1848
        assert_eq
        push.7149
        assert_eq
        push.3055
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.7918
        assert_eq
        push.10759
        assert_eq
        push.109
        assert_eq
        push.12190
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11481
        assert_eq
        push.8212
        assert_eq
        push.8442
        assert_eq
        push.7749
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5696
        assert_eq
        push.2374
        assert_eq
        push.5884
        assert_eq
        push.1367
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12273
        assert_eq
        push.3517
        assert_eq
        push.7168
        assert_eq
        push.9618
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.247
        assert_eq
        push.5187
        assert_eq
        push.7405
        assert_eq
        push.179
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.7272
        assert_eq
        push.8204
        assert_eq
        push.3469
        assert_eq
        push.2783
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11814
        assert_eq
        push.9921
        assert_eq
        push.2753
        assert_eq
        push.3246
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.10636
        assert_eq
        push.11394
        assert_eq
        push.2028
        assert_eq
        push.7508
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.2506
        assert_eq
        push.4768
        assert_eq
        push.2304
        assert_eq
        push.5062
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.3899
        assert_eq
        push.6804
        assert_eq
        push.4449
        assert_eq
        push.10144
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5419
        assert_eq
        push.8029
        assert_eq
        push.2389
        assert_eq
        push.810
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12251
        assert_eq
        push.757
        assert_eq
        push.2405
        assert_eq
        push.1716
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.2837
        assert_eq
        push.12152
        assert_eq
        push.8028
        assert_eq
        push.8055
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.6032
        assert_eq
        push.4064
        assert_eq
        push.131
        assert_eq
        push.11778
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.8269
        assert_eq
        push.5749
        assert_eq
        push.6084
        assert_eq
        push.9020
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5295
        assert_eq
        push.9212
        assert_eq
        push.9402
        assert_eq
        push.2082
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.2516
        assert_eq
        push.1388
        assert_eq
        push.2980
        assert_eq
        push.9119
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.6922
        assert_eq
        push.7261
        assert_eq
        push.3082
        assert_eq
        push.3937
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.8030
        assert_eq
        push.3419
        assert_eq
        push.11642
        assert_eq
        push.3461
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.3280
        assert_eq
        push.8021
        assert_eq
        push.12275
        assert_eq
        push.3723
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.6648
        assert_eq
        push.1899
        assert_eq
        push.3517
        assert_eq
        push.7930
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11250
        assert_eq
        push.5780
        assert_eq
        push.5186
        assert_eq
        push.10416
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.2986
        assert_eq
        push.10229
        assert_eq
        push.12223
        assert_eq
        push.1546
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.3882
        assert_eq
        push.6142
        assert_eq
        push.5681
        assert_eq
        push.2195
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.9191
        assert_eq
        push.3862
        assert_eq
        push.10569
        assert_eq
        push.6215
        assert_eq
		add.1

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11099
        assert_eq
        push.3551
        assert_eq
        push.359
        assert_eq
        push.12233
        assert_eq
		add.1

        push.0.0.0.0
        movup.4
        mem_loadw
        push.4458
        assert_eq
        push.904
        assert_eq
        push.1331
        assert_eq
        push.10946
        assert_eq
    end

    begin
        exec.wrapper
    end
    ";

    let test = build_test!(source, &[]);
    test.get_last_stack_state();
}

#[test]
fn test_poly512_sub_zq() {
    let source = "
    use.std::math::poly512

    proc.wrapper.384
        # prepare first polynomial `f`

        push.7618.7764.7271.4394
        loc_storew.127
        dropw

        push.240.9007.7416.2384
        loc_storew.126
        dropw

        push.151.696.5752.9855
        loc_storew.125
        dropw

        push.11254.226.6491.7068
        loc_storew.124
        dropw

        push.10516.11999.4160.8221
        loc_storew.123
        dropw

        push.5661.2131.1543.1886
        loc_storew.122
        dropw


        push.10731.11960.10244.5368
        loc_storew.121
        dropw


        push.1223.5240.4765.9963
        loc_storew.120
        dropw


        push.10751.2666.9203.7421
        loc_storew.119
        dropw


        push.8385.360.12030.6617
        loc_storew.118
        dropw


        push.2200.1559.7969.7859
        loc_storew.117
        dropw


        push.1688.5958.1035.1013
        loc_storew.116
        dropw


        push.6134.5570.5407.6433
        loc_storew.115
        dropw


        push.8960.2113.318.5227
        loc_storew.114
        dropw


        push.10371.11650.6156.5958
        loc_storew.113
        dropw


        push.3686.2823.11955.8012
        loc_storew.112
        dropw


        push.6769.9419.3993.4488
        loc_storew.111
        dropw


        push.81.190.3011.11793
        loc_storew.110
        dropw


        push.9463.3396.2171.3566
        loc_storew.109
        dropw


        push.10744.852.10397.2509
        loc_storew.108
        dropw


        push.10901.9641.11403.4222
        loc_storew.107
        dropw


        push.5217.7112.8609.9784
        loc_storew.106
        dropw


        push.8544.1738.3735.11320
        loc_storew.105
        dropw


        push.11983.7633.734.2530
        loc_storew.104
        dropw


        push.5079.3436.811.9673
        loc_storew.103
        dropw


        push.8968.5560.12079.9088
        loc_storew.102
        dropw


        push.2836.7454.4608.862
        loc_storew.101
        dropw


        push.639.6743.1732.10708
        loc_storew.100
        dropw


        push.4875.4161.6301.9212
        loc_storew.99
        dropw

        push.11218.67.11674.11861
        loc_storew.98
        dropw

        push.7210.5869.5014.6718
        loc_storew.97
        dropw

        push.2706.380.2286.3909
        loc_storew.96
        dropw

        push.2070.4599.6989.1000
        loc_storew.95
        dropw

        push.826.2997.165.6746
        loc_storew.94
        dropw

        push.3094.391.2166.7591
        loc_storew.93
        dropw

        push.532.1595.11816.195
        loc_storew.92
        dropw

        push.10009.8671.9088.6851
        loc_storew.91
        dropw

        push.522.11178.5937.3377
        loc_storew.90
        dropw

        push.573.12185.9043.8081
        loc_storew.89
        dropw

        push.3058.11401.7664.6180
        loc_storew.88
        dropw

        push.348.9627.4467.8534
        loc_storew.87
        dropw

        push.4289.5381.1181.11304
        loc_storew.86
        dropw

        push.6045.2243.11189.7050
        loc_storew.85
        dropw

        push.5918.11542.11147.4685
        loc_storew.84
        dropw

        push.1817.6002.11775.3084
        loc_storew.83
        dropw

        push.3440.3810.7250.7448
        loc_storew.82
        dropw

        push.2387.919.2999.4003
        loc_storew.81
        dropw

        push.4436.9507.1425.6738
        loc_storew.80
        dropw

        push.9287.11417.10830.6660
        loc_storew.79
        dropw

        push.7950.1656.1297.476
        loc_storew.78
        dropw

        push.283.8597.11086.5239
        loc_storew.77
        dropw

        push.589.8488.8795.7944
        loc_storew.76
        dropw

        push.3503.9815.11322.8029
        loc_storew.75
        dropw

        push.5579.8690.7495.2906
        loc_storew.74
        dropw

        push.12064.6795.2785.6564
        loc_storew.73
        dropw

        push.3853.3671.1751.4550
        loc_storew.72
        dropw

        push.5527.3345.4434.11097
        loc_storew.71
        dropw

        push.5401.2248.8989.3040
        loc_storew.70
        dropw

        push.11125.2582.6380.5365
        loc_storew.69
        dropw

        push.11771.8238.11912.8595
        loc_storew.68
        dropw

        push.2163.8230.4250.5354
        loc_storew.67
        dropw

        push.6625.3398.1150.8063
        loc_storew.66
        dropw

        push.3488.2537.10432.3812
        loc_storew.65
        dropw

        push.10661.8269.1789.7507
        loc_storew.64
        dropw

        push.5770.6350.5495.8113
        loc_storew.63
        dropw

        push.2282.737.650.10641
        loc_storew.62
        dropw

        push.9439.10085.3053.5555
        loc_storew.61
        dropw

        push.11982.11108.164.840
        loc_storew.60
        dropw

        push.5117.5300.4751.11981
        loc_storew.59
        dropw

        push.4584.11753.6888.3782
        loc_storew.58
        dropw

        push.3948.10542.4745.6791
        loc_storew.57
        dropw

        push.434.3061.6757.5192
        loc_storew.56
        dropw

        push.3139.8759.4495.10332
        loc_storew.55
        dropw

        push.1111.9712.9100.10891
        loc_storew.54
        dropw

        push.8147.5643.10068.5768
        loc_storew.53
        dropw

        push.2232.8988.9849.2057
        loc_storew.52
        dropw

        push.7467.1020.11978.10889
        loc_storew.51
        dropw

        push.4118.8060.5659.7640
        loc_storew.50
        dropw

        push.6317.10621.10523.755
        loc_storew.49
        dropw

        push.5309.1323.10963.886
        loc_storew.48
        dropw

        push.11317.3194.7864.8345
        loc_storew.47
        dropw

        push.3535.10043.4283.6121
        loc_storew.46
        dropw

        push.6200.8954.6105.1139
        loc_storew.45
        dropw

        push.2251.11220.6486.8163
        loc_storew.44
        dropw

        push.4813.11843.11216.10137
        loc_storew.43
        dropw

        push.10813.5679.8204.1648
        loc_storew.42
        dropw

        push.11375.8547.9702.7582
        loc_storew.41
        dropw

        push.2277.3642.6576.3166
        loc_storew.40
        dropw

        push.11404.3420.12135.6431
        loc_storew.39
        dropw

        push.9549.10272.3869.7997
        loc_storew.38
        dropw

        push.1975.2226.12018.95
        loc_storew.37
        dropw

        push.8421.11391.10714.9038
        loc_storew.36
        dropw

        push.6620.6649.3572.3789
        loc_storew.35
        dropw

        push.5219.4008.11203.9672
        loc_storew.34
        dropw

        push.6510.5476.11484.7022
        loc_storew.33
        dropw

        push.3490.7500.9936.7836
        loc_storew.32
        dropw

        push.11572.10815.10576.3575
        loc_storew.31
        dropw

        push.1421.9053.4035.8613
        loc_storew.30
        dropw

        push.9806.2186.10907.11868
        loc_storew.29
        dropw

        push.7376.9934.5963.2861
        loc_storew.28
        dropw

        push.9101.5434.10503.2109
        loc_storew.27
        dropw

        push.446.209.1679.4296
        loc_storew.26
        dropw

        push.4701.3682.4013.508
        loc_storew.25
        dropw

        push.10643.6272.10058.6575
        loc_storew.24
        dropw

        push.3016.5032.8623.12217
        loc_storew.23
        dropw

        push.91.4684.7233.53
        loc_storew.22
        dropw

        push.9335.9009.4008.5022
        loc_storew.21
        dropw

        push.9012.9274.2276.415
        loc_storew.20
        dropw

        push.5016.10207.940.1750
        loc_storew.19
        dropw

        push.7365.10035.7526.9703
        loc_storew.18
        dropw

        push.2274.7850.5694.8346
        loc_storew.17
        dropw

        push.11501.10018.4533.7010
        loc_storew.16
        dropw

        push.10479.9972.11407.12055
        loc_storew.15
        dropw

        push.4024.4270.12158.9184
        loc_storew.14
        dropw

        push.454.12075.8262.6427
        loc_storew.13
        dropw

        push.3104.6247.6381.4144
        loc_storew.12
        dropw

        push.10312.2842.2999.6860
        loc_storew.11
        dropw

        push.3072.9317.11223.9895
        loc_storew.10
        dropw

        push.8506.9148.4979.5304
        loc_storew.9
        dropw

        push.8913.623.8621.4430
        loc_storew.8
        dropw

        push.8722.94.4069.8477
        loc_storew.7
        dropw

        push.4166.9022.10574.5812
        loc_storew.6
        dropw

        push.1765.6902.6646.1069
        loc_storew.5
        dropw

        push.11125.318.2169.9207
        loc_storew.4
        dropw

        push.10176.6543.6207.8256
        loc_storew.3
        dropw

        push.5924.1719.8671.3325
        loc_storew.2
        dropw

        push.352.11961.8580.1130
        loc_storew.1
        dropw

        push.1268.10938.11332.7679
        loc_storew.0
        dropw

        # prepare second polynomial `g`

        push.7741.7665.7261.4273
        loc_storew.255
        dropw


        push.222.8745.7581.2447
        loc_storew.254
        dropw


        push.338.516.5487.9716
        loc_storew.253
        dropw


        push.11359.285.6586.6927
        loc_storew.252
        dropw


        push.10394.11968.4065.8285
        loc_storew.251
        dropw


        push.5902.2112.1450.2139
        loc_storew.250
        dropw


        push.10743.12222.10169.5205
        loc_storew.249
        dropw


        push.1081.5329.4937.10052
        loc_storew.248
        dropw


        push.10821.2675.9075.7372
        loc_storew.247
        dropw


        push.8559.396.12247.6781
        loc_storew.246
        dropw


        push.1761.1625.7749.7955
        loc_storew.245
        dropw


        push.2250.5927.756.1346
        loc_storew.244
        dropw


        push.6231.6005.5565.6472
        loc_storew.243
        dropw


        push.8943.2170.401.5228
        loc_storew.242
        dropw


        push.10128.11840.6011.6071
        loc_storew.241
        dropw


        push.3548.2877.11721.8101
        loc_storew.240
        dropw


        push.6914.9576.3816.4724
        loc_storew.239
        dropw


        push.267.315.3083.11614
        loc_storew.238
        dropw


        push.9629.3496.1881.3547
        loc_storew.237
        dropw


        push.10655.980.10403.2660
        loc_storew.236
        dropw


        push.11119.9598.11661.4179
        loc_storew.235
        dropw


        push.5155.7064.8521.9834
        loc_storew.234
        dropw


        push.8651.1707.3859.11311
        loc_storew.233
        dropw


        push.12130.7821.596.2426
        loc_storew.232
        dropw


        push.4797.3518.660.9806
        loc_storew.231
        dropw


        push.9044.5421.11879.9000
        loc_storew.230
        dropw


        push.2878.7444.4404.917
        loc_storew.229
        dropw


        push.777.6703.2011.10618
        loc_storew.228
        dropw


        push.4915.4333.6051.9360
        loc_storew.227
        dropw


        push.11143.325.11914.11693
        loc_storew.226
        dropw


        push.7200.6053.5012.6590
        loc_storew.225
        dropw


        push.2956.450.1985.3799
        loc_storew.224
        dropw


        push.2286.4582.6658.1028
        loc_storew.223
        dropw


        push.875.3006.165.6752
        loc_storew.222
        dropw


        push.3081.628.2134.7575
        loc_storew.221
        dropw


        push.460.1644.11809.12287
        loc_storew.220
        dropw


        push.9948.8751.9171.6547
        loc_storew.219
        dropw


        push.338.11149.5957.3241
        loc_storew.218
        dropw


        push.572.11899.9000.7752
        loc_storew.217
        dropw


        push.2817.11228.7751.6166
        loc_storew.216
        dropw


        push.179.9839.4532.8570
        loc_storew.215
        dropw


        push.4512.5462.996.11062
        loc_storew.214
        dropw


        push.6122.2327.11427.6848
        loc_storew.213
        dropw


        push.5857.11153.11240.4612
        loc_storew.212
        dropw


        push.2132.6301.11725.3207
        loc_storew.211
        dropw


        push.3202.3533.7255.7461
        loc_storew.210
        dropw


        push.2439.935.3119.3873
        loc_storew.209
        dropw


        push.4582.9335.1220.6826
        loc_storew.208
        dropw


        push.9401.11338.10466.6425
        loc_storew.207
        dropw


        push.8194.1649.1250.389
        loc_storew.206
        dropw


        push.262.8865.11095.5199
        loc_storew.205
        dropw


        push.492.8700.8533.8101
        loc_storew.204
        dropw


        push.3300.9703.11544.8104
        loc_storew.203
        dropw


        push.5387.8853.7458.2987
        loc_storew.202
        dropw


        push.11947.6928.2601.6423
        loc_storew.201
        dropw


        push.3629.3786.1786.4817
        loc_storew.200
        dropw


        push.5536.3446.4449.11214
        loc_storew.199
        dropw


        push.5141.2095.8984.3020
        loc_storew.198
        dropw


        push.11361.2331.6591.5348
        loc_storew.197
        dropw


        push.11891.8313.11887.8652
        loc_storew.196
        dropw


        push.1956.8371.4281.5291
        loc_storew.195
        dropw


        push.6645.3320.1159.7931
        loc_storew.194
        dropw


        push.3758.2558.10557.3675
        loc_storew.193
        dropw


        push.10651.8160.1638.7653
        loc_storew.192
        dropw


        push.5805.6552.5593.8007
        loc_storew.191
        dropw


        push.2090.805.857.10589
        loc_storew.190
        dropw


        push.9292.10230.3098.5515
        loc_storew.189
        dropw


        push.12027.11242.458.476
        loc_storew.188
        dropw


        push.4893.5208.4691.12002
        loc_storew.187
        dropw


        push.4570.11574.6957.3674
        loc_storew.186
        dropw


        push.3813.10451.4612.6559
        loc_storew.185
        dropw


        push.525.2777.6824.5224
        loc_storew.184
        dropw


        push.3033.8664.4425.10411
        loc_storew.183
        dropw


        push.983.9737.9042.10736
        loc_storew.182
        dropw


        push.8180.5959.9958.5733
        loc_storew.181
        dropw


        push.2445.8923.9845.2150
        loc_storew.180
        dropw


        push.7385.1154.12268.10855
        loc_storew.179
        dropw


        push.4142.7945.5279.7955
        loc_storew.178
        dropw


        push.6464.10540.10477.361
        loc_storew.177
        dropw


        push.5295.1524.10998.753
        loc_storew.176
        dropw


        push.11472.2866.7888.8158
        loc_storew.175
        dropw


        push.3871.9994.4479.6273
        loc_storew.174
        dropw


        push.6329.9199.6087.1385
        loc_storew.173
        dropw


        push.2173.11072.6464.8249
        loc_storew.172
        dropw


        push.4747.11994.10806.9758
        loc_storew.171
        dropw


        push.10870.5517.8112.1595
        loc_storew.170
        dropw


        push.11219.8595.9828.7465
        loc_storew.169
        dropw


        push.2276.3738.6556.3180
        loc_storew.168
        dropw


        push.11734.3480.11928.6333
        loc_storew.167
        dropw


        push.9461.10375.3853.8004
        loc_storew.166
        dropw


        push.2119.2252.11823.270
        loc_storew.165
        dropw


        push.8675.11329.10642.8782
        loc_storew.164
        dropw


        push.6307.6708.3657.3722
        loc_storew.163
        dropw


        push.4956.4138.11020.9757
        loc_storew.162
        dropw


        push.6339.5377.11644.6737
        loc_storew.161
        dropw


        push.3354.7725.10029.7792
        loc_storew.160
        dropw


        push.11773.10899.10305.3477
        loc_storew.159
        dropw


        push.1365.8967.4005.8545
        loc_storew.158
        dropw


        push.9758.1919.10647.12028
        loc_storew.157
        dropw


        push.7332.9785.5966.2857
        loc_storew.156
        dropw


        push.9234.5140.10441.1853
        loc_storew.155
        dropw


        push.99.12180.1530.4371
        loc_storew.154
        dropw


        push.4540.3847.4077.808
        loc_storew.153
        dropw


        push.10922.6405.9915.6593
        loc_storew.152
        dropw


        push.2671.5121.8772.16
        loc_storew.151
        dropw


        push.12110.4884.7102.12042
        loc_storew.150
        dropw


        push.9506.8820.4085.5017
        loc_storew.149
        dropw


        push.9043.9536.2368.475
        loc_storew.148
        dropw


        push.4781.10261.895.1653
        loc_storew.147
        dropw


        push.7227.9985.7521.9783
        loc_storew.146
        dropw


        push.2145.7840.5485.8390
        loc_storew.145
        dropw


        push.11479.9900.4260.6870
        loc_storew.144
        dropw


        push.10573.9884.11532.38
        loc_storew.143
        dropw


        push.4234.4261.137.9452
        loc_storew.142
        dropw


        push.511.12158.8225.6257
        loc_storew.141
        dropw


        push.3269.6205.6540.4020
        loc_storew.140
        dropw


        push.10207.2887.3077.6994
        loc_storew.139
        dropw


        push.3170.9309.10901.9773
        loc_storew.138
        dropw


        push.8352.9207.5028.5367
        loc_storew.137
        dropw


        push.8828.647.8870.4259
        loc_storew.136
        dropw


        push.8566.14.4268.9009
        loc_storew.135
        dropw


        push.4359.8772.10390.5641
        loc_storew.134
        dropw


        push.1873.7103.6509.1039
        loc_storew.133
        dropw


        push.10743.66.2060.9303
        loc_storew.132
        dropw


        push.10094.6608.6147.8407
        loc_storew.131
        dropw


        push.6074.1720.8427.3098
        loc_storew.130
        dropw


        push.56.11930.8738.1190
        loc_storew.129
        dropw


        push.1343.10958.11385.7831
        loc_storew.128
        dropw


        # prepare argument ( absolute memory addresses ) for subtracting two polynomials

        locaddr.383 # output
        locaddr.255 # input 1
        locaddr.127 # input 0

        # perform polynomial subtraction, when two polynomials are provided
        # as absolute memory addresses on the stack

        exec.poly512::sub_zq

        # check for functional correctness ( using known answer test )

        locaddr.383

		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.121
        assert_eq
        push.10
        assert_eq
        push.99
        assert_eq
        push.12166
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12226
        assert_eq
        push.12124
        assert_eq
        push.262
        assert_eq
        push.18
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.139
        assert_eq
        push.265
        assert_eq
        push.180
        assert_eq
        push.12102
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.141
        assert_eq
        push.12194
        assert_eq
        push.12230
        assert_eq
        push.12184
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12225
        assert_eq
        push.95
        assert_eq
        push.31
        assert_eq
        push.122
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12036
        assert_eq
        push.93
        assert_eq
        push.19
        assert_eq
        push.12048
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.163
        assert_eq
        push.75
        assert_eq
        push.12027
        assert_eq
        push.12277
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12200
        assert_eq
        push.12117
        assert_eq
        push.12200
        assert_eq
        push.142
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.49
        assert_eq
        push.128
        assert_eq
        push.12280
        assert_eq
        push.12219
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12125
        assert_eq
        push.12072
        assert_eq
        push.12253
        assert_eq
        push.12115
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12193
        assert_eq
        push.220
        assert_eq
        push.12223
        assert_eq
        push.439
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11956
        assert_eq
        push.279
        assert_eq
        push.31
        assert_eq
        push.11727
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12250
        assert_eq
        push.12131
        assert_eq
        push.11854
        assert_eq
        push.12192
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12288
        assert_eq
        push.12206
        assert_eq
        push.12232
        assert_eq
        push.17
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12176
        assert_eq
        push.145
        assert_eq
        push.12099
        assert_eq
        push.243
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12200
        assert_eq
        push.234
        assert_eq
        push.12235
        assert_eq
        push.138
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12053
        assert_eq
        push.177
        assert_eq
        push.12132
        assert_eq
        push.12144
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.179
        assert_eq
        push.12217
        assert_eq
        push.12164
        assert_eq
        push.12103
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.19
        assert_eq
        push.290
        assert_eq
        push.12189
        assert_eq
        push.12123
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12138
        assert_eq
        push.12283
        assert_eq
        push.12161
        assert_eq
        push.89
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.43
        assert_eq
        push.12031
        assert_eq
        push.43
        assert_eq
        push.12071
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12239
        assert_eq
        push.88
        assert_eq
        push.48
        assert_eq
        push.62
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.9
        assert_eq
        push.12165
        assert_eq
        push.31
        assert_eq
        push.12182
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.104
        assert_eq
        push.138
        assert_eq
        push.12101
        assert_eq
        push.12142
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12156
        assert_eq
        push.151
        assert_eq
        push.12207
        assert_eq
        push.282
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.88
        assert_eq
        push.200
        assert_eq
        push.139
        assert_eq
        push.12213
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12234
        assert_eq
        push.204
        assert_eq
        push.10
        assert_eq
        push.12247
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.90
        assert_eq
        push.12010
        assert_eq
        push.40
        assert_eq
        push.12151
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12141
        assert_eq
        push.250
        assert_eq
        push.12117
        assert_eq
        push.12249
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.168
        assert_eq
        push.12049
        assert_eq
        push.12031
        assert_eq
        push.75
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.128
        assert_eq
        push.2
        assert_eq
        push.12105
        assert_eq
        push.10
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.110
        assert_eq
        push.301
        assert_eq
        push.12219
        assert_eq
        push.12039
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12261
        assert_eq
        push.331
        assert_eq
        push.17
        assert_eq
        push.12073
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12283
        assert_eq
        push.0
        assert_eq
        push.12280
        assert_eq
        push.12240
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.16
        assert_eq
        push.32
        assert_eq
        push.12052
        assert_eq
        push.13
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.197
        assert_eq
        push.7
        assert_eq
        push.12240
        assert_eq
        push.72
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.304
        assert_eq
        push.12206
        assert_eq
        push.12209
        assert_eq
        push.61
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.136
        assert_eq
        push.12269
        assert_eq
        push.29
        assert_eq
        push.184
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.329
        assert_eq
        push.43
        assert_eq
        push.286
        assert_eq
        push.1
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.14
        assert_eq
        push.12202
        assert_eq
        push.173
        assert_eq
        push.241
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12253
        assert_eq
        push.12224
        assert_eq
        push.12077
        assert_eq
        push.169
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.242
        assert_eq
        push.185
        assert_eq
        push.12208
        assert_eq
        push.12066
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.202
        assert_eq
        push.12051
        assert_eq
        push.12205
        assert_eq
        push.12212
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.73
        assert_eq
        push.12196
        assert_eq
        push.389
        assert_eq
        push.61
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12166
        assert_eq
        push.50
        assert_eq
        push.11990
        assert_eq
        push.11974
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12276
        assert_eq
        push.12284
        assert_eq
        push.277
        assert_eq
        push.238
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.130
        assert_eq
        push.12169
        assert_eq
        push.12273
        assert_eq
        push.12237
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12201
        assert_eq
        push.205
        assert_eq
        push.172
        assert_eq
        push.12143
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.235
        assert_eq
        push.364
        assert_eq
        push.79
        assert_eq
        push.12175
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.87
        assert_eq
        push.47
        assert_eq
        push.7
        assert_eq
        push.12045
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.40
        assert_eq
        push.12280
        assert_eq
        push.12021
        assert_eq
        push.21
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12132
        assert_eq
        push.262
        assert_eq
        push.12077
        assert_eq
        push.97
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12214
        assert_eq
        push.12067
        assert_eq
        push.112
        assert_eq
        push.203
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12208
        assert_eq
        push.37
        assert_eq
        push.12126
        assert_eq
        push.192
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.141
        assert_eq
        push.184
        assert_eq
        push.12156
        assert_eq
        push.117
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12022
        assert_eq
        push.12254
        assert_eq
        push.12174
        assert_eq
        push.224
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12172
        assert_eq
        push.12274
        assert_eq
        push.12188
        assert_eq
        push.12280
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.20
        assert_eq
        push.5
        assert_eq
        push.153
        assert_eq
        push.260
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.17
        assert_eq
        push.12078
        assert_eq
        push.251
        assert_eq
        push.12053
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12232
        assert_eq
        push.25
        assert_eq
        push.12214
        assert_eq
        push.12169
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.63
        assert_eq
        push.12258
        assert_eq
        push.12148
        assert_eq
        push.207
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.132
        assert_eq
        push.12280
        assert_eq
        push.78
        assert_eq
        push.12269
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.137
        assert_eq
        push.12164
        assert_eq
        push.12268
        assert_eq
        push.12019
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12143
        assert_eq
        push.151
        assert_eq
        push.109
        assert_eq
        push.10
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.106
        assert_eq
        push.12191
        assert_eq
        push.12087
        assert_eq
        push.12254
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.52
        assert_eq
        push.12082
        assert_eq
        push.12221
        assert_eq
        push.192
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.40
        assert_eq
        push.12244
        assert_eq
        push.12144
        assert_eq
        push.147
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.364
        assert_eq
        push.11995
        assert_eq
        push.12155
        assert_eq
        push.12244
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12268
        assert_eq
        push.60
        assert_eq
        push.92
        assert_eq
        push.224
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.108
        assert_eq
        push.12220
        assert_eq
        push.179
        assert_eq
        push.14
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.232
        assert_eq
        push.133
        assert_eq
        push.91
        assert_eq
        push.135
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12257
        assert_eq
        push.12222
        assert_eq
        push.284
        assert_eq
        push.12198
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12210
        assert_eq
        push.70
        assert_eq
        push.95
        assert_eq
        push.106
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.155
        assert_eq
        push.58
        assert_eq
        push.12264
        assert_eq
        push.128
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.35
        assert_eq
        push.110
        assert_eq
        push.11973
        assert_eq
        push.12256
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12196
        assert_eq
        push.4
        assert_eq
        push.65
        assert_eq
        push.12076
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.34
        assert_eq
        push.11999
        assert_eq
        push.12155
        assert_eq
        push.82
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11974
        assert_eq
        push.380
        assert_eq
        push.115
        assert_eq
        push.12265
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.394
        assert_eq
        push.46
        assert_eq
        push.81
        assert_eq
        push.12142
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.133
        assert_eq
        push.12254
        assert_eq
        push.12088
        assert_eq
        push.14
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.187
        assert_eq
        push.12265
        assert_eq
        push.328
        assert_eq
        push.12134
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12137
        assert_eq
        push.12093
        assert_eq
        push.49
        assert_eq
        push.11953
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12043
        assert_eq
        push.18
        assert_eq
        push.12044
        assert_eq
        push.12160
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12203
        assert_eq
        push.22
        assert_eq
        push.148
        assert_eq
        push.78
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.379
        assert_eq
        push.410
        assert_eq
        push.12138
        assert_eq
        push.66
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.53
        assert_eq
        push.92
        assert_eq
        push.162
        assert_eq
        push.12232
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.117
        assert_eq
        push.12163
        assert_eq
        push.12241
        assert_eq
        push.156
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12275
        assert_eq
        push.20
        assert_eq
        push.12193
        assert_eq
        push.1
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.98
        assert_eq
        push.207
        assert_eq
        push.12229
        assert_eq
        push.11959
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12282
        assert_eq
        push.16
        assert_eq
        push.12186
        assert_eq
        push.88
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12114
        assert_eq
        push.195
        assert_eq
        push.12263
        assert_eq
        push.12145
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.256
        assert_eq
        push.72
        assert_eq
        push.62
        assert_eq
        push.12035
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.67
        assert_eq
        push.12204
        assert_eq
        push.12230
        assert_eq
        push.313
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12204
        assert_eq
        push.183
        assert_eq
        push.12159
        assert_eq
        push.263
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.285
        assert_eq
        push.12129
        assert_eq
        push.99
        assert_eq
        push.171
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.44
        assert_eq
        push.12196
        assert_eq
        push.12064
        assert_eq
        push.136
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.98
        assert_eq
        push.271
        assert_eq
        push.12205
        assert_eq
        push.12088
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.68
        assert_eq
        push.30
        assert_eq
        push.86
        assert_eq
        push.56
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12129
        assert_eq
        push.260
        assert_eq
        push.267
        assert_eq
        push.48
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.4
        assert_eq
        push.12286
        assert_eq
        push.149
        assert_eq
        push.44
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.256
        assert_eq
        push.62
        assert_eq
        push.294
        assert_eq
        push.12156
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12214
        assert_eq
        push.149
        assert_eq
        push.318
        assert_eq
        push.347
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11989
        assert_eq
        push.12225
        assert_eq
        push.12124
        assert_eq
        push.161
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12271
        assert_eq
        push.143
        assert_eq
        push.12156
        assert_eq
        push.12010
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12201
        assert_eq
        push.12140
        assert_eq
        push.12200
        assert_eq
        push.345
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.300
        assert_eq
        push.131
        assert_eq
        push.12089
        assert_eq
        push.270
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.5
        assert_eq
        push.12212
        assert_eq
        push.189
        assert_eq
        push.12118
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12229
        assert_eq
        push.12197
        assert_eq
        push.12027
        assert_eq
        push.12258
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.97
        assert_eq
        push.45
        assert_eq
        push.12235
        assert_eq
        push.235
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12209
        assert_eq
        push.5
        assert_eq
        push.50
        assert_eq
        push.138
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12245
        assert_eq
        push.209
        assert_eq
        push.10
        assert_eq
        push.129
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.140
        assert_eq
        push.273
        assert_eq
        push.118
        assert_eq
        push.22
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12017
        assert_eq
        push.12164
        assert_eq
        push.88
        assert_eq
        push.12195
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12021
        assert_eq
        push.12021
        assert_eq
        push.9
        assert_eq
        push.12079
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.170
        assert_eq
        push.37
        assert_eq
        push.12206
        assert_eq
        push.12232
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.124
        assert_eq
        push.12130
        assert_eq
        push.42
        assert_eq
        push.12124
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12155
        assert_eq
        push.12211
        assert_eq
        push.12244
        assert_eq
        push.105
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.122
        assert_eq
        push.322
        assert_eq
        push.8
        assert_eq
        push.12191
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12226
        assert_eq
        push.12240
        assert_eq
        push.12230
        assert_eq
        push.154
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.171
        assert_eq
        push.12040
        assert_eq
        push.12265
        assert_eq
        push.85
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.11757
        assert_eq
        push.12090
        assert_eq
        push.80
        assert_eq
        push.156
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.171
        assert_eq
        push.184
        assert_eq
        push.250
        assert_eq
        push.12096
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.30
        assert_eq
        push.137
        assert_eq
        push.12088
        assert_eq
        push.12181
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12193
        assert_eq
        push.109
        assert_eq
        push.252
        assert_eq
        push.382
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12138
        assert_eq
        push.60
        assert_eq
        push.12224
        assert_eq
        push.82
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.227
        assert_eq
        push.244
        assert_eq
        push.12288
        assert_eq
        push.12139
        assert_eq
		add.1
    
		dup
		push.0.0.0.0
        movup.4
        mem_loadw
        push.12229
        assert_eq
        push.12131
        assert_eq
        push.31
        assert_eq
        push.296
        assert_eq
		add.1
    
        push.0.0.0.0
        movup.4
        mem_loadw
        push.12137
        assert_eq
        push.12236
        assert_eq
        push.12269
        assert_eq
        push.12214
        assert_eq
    end

    begin
        exec.wrapper
    end
    ";

    let test = build_test!(source, &[]);
    test.get_last_stack_state();
}