from sklearn.datasets import load_iris
from sklearn.linear_model import LogisticRegression
import pandas as pd

X, y = load_iris(return_X_y=True)
clf = LogisticRegression(random_state=0).fit(X, y)
print(clf.score(X, y))

data = load_iris()
df = pd.DataFrame(data.data, columns=data.feature_names)
df['target'] = data.target
df.to_csv('iris.csv')
