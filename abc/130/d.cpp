#include <iostream>
#include <vector>
typedef long long ll;
using namespace std;

int main()
{
    ll n, k;
    cin >> n >> k;

    vector<ll> a;
    ll summary = 0;
    for (ll i = 0; i < n; ++i)
    {
        ll tmp;
        cin >> tmp;
        summary += tmp;
        a.push_back(tmp);
    }

    ll cnt = 0;
    for (ll left = 0; left < n; ++left)
    {
        ll tmp = summary;
        for (ll right = n; right >= left + 1; --right)
        {
            if (tmp >= k)
            {
                cnt++;
                if (right - 1 >= 0)
                {
                    tmp -= a[right - 1];
                }
            }
            else
            {
                break;
            }
        }
        summary -= a[left];
        if (summary < k)
        {
            break;
        }
    }

    cout << cnt << endl;
    return 0;
}
